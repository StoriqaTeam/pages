#![recursion_limit = "128"]
extern crate chrono;
extern crate config as config_crate;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate diesel;
extern crate env_logger;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate futures_cpupool;
extern crate hyper;
extern crate hyper_tls;
#[macro_use]
extern crate log;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate regex;
#[macro_use]
extern crate serde;
extern crate serde_json;
extern crate stq_api;
extern crate stq_db;
extern crate stq_http;
extern crate stq_logging;
extern crate stq_roles;
extern crate stq_router;
extern crate stq_static_resources;
extern crate stq_types;
extern crate tokio_core;
extern crate uuid;

pub mod config;
mod controller;
mod errors;
mod models;
mod repos;
mod schema;
mod services;

pub use config::Config;
use errors::Error;

use diesel::pg::PgConnection;
use futures::future;
use futures::{Future, Stream};
use futures_cpupool::CpuPool;
use hyper::server::Http;
use r2d2_diesel::ConnectionManager;
use std::process;
use std::sync::Arc;
use stq_http::controller::Application;
use tokio_core::reactor::Core;

/// Starts new web service from provided `Config`
pub fn start_server<F: FnOnce() + 'static>(config: Config, port: Option<u16>, callback: F) {
    // Prepare reactor
    let mut core = Core::new().expect("Unexpected error creating event loop core");
    let handle = Arc::new(core.handle());

    // Prepare database pool
    let database_url: String = config
        .db
        .dsn
        .parse()
        .expect("Database URL must be set in configuration");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let r2d2_pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool");

    let thread_count = config.server.thread_count;

    // Prepare CPU pool
    let cpu_pool = CpuPool::new(thread_count);

    // Prepare server
    let address = {
        let port = port.as_ref().unwrap_or(&config.server.port);
        format!("{}:{}", config.server.host, port)
            .parse()
            .expect("Could not parse address")
    };

    let serve = Http::new()
        .serve_addr_handle(&address, &handle, move || {
            let controller = controller::ControllerImpl::new(r2d2_pool.clone(), cpu_pool.clone());

            // Prepare application
            let app = Application::<Error>::new(controller);

            Ok(app)
        })
        .unwrap_or_else(|why| {
            error!("Http Server Initialization Error: {}", why);
            process::exit(1);
        });

    handle.spawn(
        serve
            .for_each({
                let handle = handle.clone();
                move |conn| {
                    handle.spawn(
                        conn.map(|_| ())
                            .map_err(|why| error!("Server Error: {}", why)),
                    );
                    Ok(())
                }
            })
            .map_err(|_| ()),
    );

    info!("Listening on http://{}, threads: {}", address, thread_count);
    handle.spawn_fn(move || {
        callback();
        future::ok(())
    });
    core.run(future::empty::<(), ()>()).unwrap();
}
