use diesel::{connection::AnsiTransactionManager, pg::Pg, Connection};
use failure::Fail;
use futures::prelude::*;
use futures_cpupool::CpuPool;
use r2d2::{ManageConnection, Pool};
use repos::ReposFactory;
use stq_api::pages::*;
use stq_types::*;

use super::ServiceFuture;
use errors::Error;

pub trait PageService: Send + Sync {
    fn get_page(&self, identifier: PageIdentifier) -> ServiceFuture<Option<Page>>;
    fn insert_page(&self, item: NewPage) -> ServiceFuture<Page>;
}

pub struct PageServiceImpl<
    T: Connection<Backend = Pg, TransactionManager = AnsiTransactionManager> + 'static,
    M: ManageConnection<Connection = T>,
    F: ReposFactory<T>,
> {
    pub db_pool: Pool<M>,
    pub cpu_pool: CpuPool,
    pub repo_factory: F,
}

impl<
        T: Connection<Backend = Pg, TransactionManager = AnsiTransactionManager> + 'static,
        M: ManageConnection<Connection = T>,
        F: ReposFactory<T>,
    > PageService for PageServiceImpl<T, M, F>
{
    fn get_page(&self, identifier: PageIdentifier) -> ServiceFuture<Option<Page>> {
        let db_pool = self.db_pool.clone();
        let repo_factory = self.repo_factory.clone();

        Box::new(
            self.cpu_pool
                .spawn_fn(move || {
                    db_pool
                        .get()
                        .map_err(|e| e.context(Error::Connection).into())
                        .and_then(move |conn| {
                            let repo = repo_factory.create_pages_repo(&*conn);
                            match identifier {
                                PageIdentifier::Id(id) => repo.find(id),
                                PageIdentifier::Slug(slug) => repo.find_by_slug(slug),
                            }
                        })
                })
                .map_err(|e| e.context("Failed to get page").into()),
        )
    }

    fn insert_page(&self, new_page: NewPage) -> ServiceFuture<Page> {
        let db_pool = self.db_pool.clone();
        let repo_factory = self.repo_factory.clone();

        Box::new(
            self.cpu_pool
                .spawn_fn(move || {
                    db_pool
                        .get()
                        .map_err(|e| e.context(Error::Connection).into())
                        .and_then(move |conn| {
                            let repo = repo_factory.create_pages_repo(&*conn);
                            repo.create(new_page)
                        })
                })
                .map_err(|e| e.context("Failed to insert page").into()),
        )
    }
}
