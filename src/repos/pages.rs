use schema::pages::dsl::*;

use diesel::{
    self, connection::AnsiTransactionManager, pg::Pg, prelude::*, query_dsl::RunQueryDsl,
    Connection,
};
use failure::{self, Fallible};
use models::page::*;
use stq_api::pages::*;
use stq_db::diesel_repo::*;
use stq_types::{PageId, PageSlug};

pub trait PagesRepo {
    fn find(&self, id: PageId) -> Fallible<Option<Page>>;
    fn find_by_slug(&self, slug: PageSlug) -> Fallible<Option<Page>>;
    fn create(&self, item: NewPage) -> Fallible<Page>;
}

impl<'a, T: Connection<Backend = Pg, TransactionManager = AnsiTransactionManager> + 'static>
    PagesRepo for DieselRepoImpl<'a, T, ()>
{
    fn find(&self, id_arg: PageId) -> Fallible<Option<Page>> {
        pages
            .find(id_arg)
            .get_result(self.db_conn)
            .map(|v: DbPage| v.into())
            .optional()
            .map_err(From::from)
    }
    fn find_by_slug(&self, slug_arg: PageSlug) -> Fallible<Option<Page>> {
        pages
            .filter(slug.eq(slug_arg))
            .get_result(self.db_conn)
            .map(|v: DbPage| v.into())
            .optional()
            .map_err(From::from)
    }
    fn create(&self, item: NewPage) -> Fallible<Page> {
        diesel::insert_into(pages)
            .values(&DbNewPage::from(item.clone()))
            .get_result::<DbPage>(self.db_conn)
            .map(From::from)
            .map_err(|e| {
                failure::Error::from(e)
                    .context(format!("Failed to create page {:?}", item))
                    .into()
            })
    }
}
