use errors::*;
use repos::*;
use sentry_integration::log_and_capture_error;
use services::*;

use diesel::{connection::AnsiTransactionManager, pg::Pg, Connection};
use failure::Fail;
use futures::{future, prelude::*};
use futures_cpupool::CpuPool;
use hyper::header::Authorization;
use hyper::server::Request;
use hyper::{Get, Post};
use r2d2::{ManageConnection, Pool};
use std::str::FromStr;
use std::sync::Arc;
use stq_api::pages::*;
use stq_http::{
    controller::{Controller, ControllerFuture},
    errors::ErrorMessageWrapper,
    request_util::{parse_body, serialize_future},
};
use stq_types::*;

/// Controller handles route parsing and calling `Service` layer
#[derive(Clone)]
pub struct ControllerImpl {
    pub service_factory: Arc<Fn() -> Box<PageService>>,
}

impl ControllerImpl {
    /// Create a new controller based on services
    pub fn new<T: Connection<Backend = Pg, TransactionManager = AnsiTransactionManager> + 'static, M: ManageConnection<Connection = T>>(
        db_pool: Pool<M>,
        cpu_pool: CpuPool,
    ) -> Self {
        Self {
            service_factory: Arc::new(move || {
                Box::new(PageServiceImpl {
                    db_pool: db_pool.clone(),
                    cpu_pool: cpu_pool.clone(),
                    repo_factory: ReposFactoryImpl,
                })
            }),
        }
    }
}

impl Controller for ControllerImpl {
    /// Handle a request and get future response
    fn call(&self, req: Request) -> ControllerFuture {
        let headers = req.headers().clone();
        let auth_header = headers.get::<Authorization<String>>();
        let user_id = auth_header.map(|auth| auth.0.clone()).and_then(|id| UserId::from_str(&id).ok());

        debug!("User with id = '{:?}' is requesting {}", user_id, req.path());

        let service = (self.service_factory)();

        let path = req.path().to_string();

        let fut = match (req.method().clone(), Route::from_path(&path)) {
            (Get, Some(Route::Page { identifier })) => serialize_future(service.get_page(identifier)),
            (Post, Some(Route::Pages)) => serialize_future(
                parse_body::<NewPage>(req.body())
                    .map_err(|e| e.context("Failed to parse NewPage body").context(Error::Parse).into())
                    .and_then(move |new_page| service.insert_page(new_page)),
            ),

            // Fallback
            (m, _) => Box::new(future::err(
                format_err!("Request to invalid endpoint in microservice! {:?} {:?}", m, path)
                    .context(Error::NotFound)
                    .into(),
            )),
        }.map_err(|err| {
            let wrapper = ErrorMessageWrapper::<Error>::from(&err);
            if wrapper.inner.code == 500 {
                log_and_capture_error(&err);
            }
            err
        });

        Box::new(fut)
    }
}
