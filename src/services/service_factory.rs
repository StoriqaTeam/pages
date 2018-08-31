pub trait ServicesFactory {
    fn create_pages_service(&self) -> Box<PagesService>;
}

#[derive(Clone, Default)]
pub struct ServicesFactoryImpl {

}

impl ServicesFactory for ServicesFactoryImpl
{
    fn create_pages_service(&self) -> Box<PagesService> {
        Box::new(DieselRepoImpl::new(db_conn)) as Box<PagesService>
    }
}
