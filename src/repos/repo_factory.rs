use diesel::connection::AnsiTransactionManager;
use diesel::pg::Pg;
use diesel::Connection;
use stq_db::diesel_repo::*;

use repos::*;

pub trait ReposFactory<
    C: Connection<Backend = Pg, TransactionManager = AnsiTransactionManager> + 'static,
>: Clone + Send + Sync + 'static
{
    fn create_pages_repo<'a>(&self, db_conn: &'a C) -> Box<PagesRepo + 'a>;
}

#[derive(Clone, Default)]
pub struct ReposFactoryImpl;

impl<C: Connection<Backend = Pg, TransactionManager = AnsiTransactionManager> + 'static>
    ReposFactory<C> for ReposFactoryImpl
{
    fn create_pages_repo<'a>(&self, db_conn: &'a C) -> Box<PagesRepo + 'a> {
        Box::new(DieselRepoImpl::new(db_conn)) as Box<PagesRepo>
    }
}
