use super::model;

use std::borrow::Cow;

use anyhow::Result;
use anyhow::Context;

pub trait CatalogProvider {
    fn get_catalog(&self) -> Result<Cow<model::Catalog>>;

    fn to_single(&self) -> Result<SingleCatalogProvider> {
        self.get_catalog()
            .map(|cow| match cow {
                Cow::Owned(catalog)        => catalog,
                Cow::Borrowed(catalog)     => catalog.clone(),
            })
            .map(|catalog| SingleCatalogProvider::new(catalog))
    }
}

#[derive(Clone)]
pub struct SingleCatalogProvider {
    catalog: model::Catalog,
}

impl SingleCatalogProvider {

    pub fn new(catalog: model::Catalog) -> Self {
        Self {
            catalog,
        }
    }
}

impl CatalogProvider for SingleCatalogProvider {
    fn get_catalog(&self) -> Result<Cow<model::Catalog>> {
        Ok(Cow::Borrowed(&self.catalog))
    }
}

#[derive(Clone)]
pub struct JsonFileCatalogProvider {
    path: String,
}

impl JsonFileCatalogProvider {
    pub fn new(path: &str) -> Self {
        JsonFileCatalogProvider {
            path: path.to_owned()
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

impl CatalogProvider for JsonFileCatalogProvider {
    fn get_catalog(&self) -> Result<Cow<model::Catalog>> {
        let path = self.path();
        let file = std::fs::File::open(path)
                                 .with_context(|| format!("Access to catalog file '{}' has failed", path))?;
        let catalog: model::Catalog = serde_json::from_reader(file)
                                                 .with_context(|| format!("Can't read catalog file '{}' as JSON", path))?;
        Ok(Cow::Owned(catalog))
    }
}

pub struct CachingCatalogProvider<T: CatalogProvider> {
    provider: T,
    cache: std::cell::RefCell<Option<SingleCatalogProvider>>,
}

impl<T: CatalogProvider> CachingCatalogProvider<T> {
    pub fn new(provider: T) -> Self {
        CachingCatalogProvider {
            provider,
            cache: std::cell::RefCell::default(),
        }
    }
}

impl<T: CatalogProvider> CatalogProvider for CachingCatalogProvider<T> {
    fn get_catalog(&self) -> Result<Cow<model::Catalog>> {
        if let Some(provider) = self.cache.borrow().as_ref() {
            let catalog = provider.get_catalog()?;
            return Ok(Cow::Owned(catalog.into_owned()))
        }
        let caching = self.provider.to_single()?;
        *self.cache.borrow_mut() = Some(caching);
        self.get_catalog()
    }
}

pub mod providers {
    pub mod catalog {
        use super::super::{model, CatalogProvider, SingleCatalogProvider, JsonFileCatalogProvider, CachingCatalogProvider};

        pub fn single(catalog: model::Catalog) -> SingleCatalogProvider {
            SingleCatalogProvider::new(catalog)
        }

        pub fn file_json(path: &str) -> JsonFileCatalogProvider {
            JsonFileCatalogProvider::new(path)
        }

        pub fn cache<T: CatalogProvider>(provider: T) -> CachingCatalogProvider<T> {
            CachingCatalogProvider::new(provider)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{model, CatalogProvider, SingleCatalogProvider, JsonFileCatalogProvider, CachingCatalogProvider};
    use anyhow::Result;

    fn build_catalog() -> model::Catalog {
        let mut catalog = model::Catalog::new();


        let mut mysql = model::Service::new();
        *mysql.id_mut() = "mysql".to_owned();
        *mysql.name_mut() = "MySQL".to_owned();

        let mut mysql_free = model::ServicePlan::new();
        *mysql_free.id_mut() = "mysql_free".to_owned();
        *mysql_free.name_mut() = "MySQL (Free)".to_owned();
        mysql.plans_mut().push(mysql_free);

        let mut mysql_small = model::ServicePlan::new();
        *mysql_small.id_mut() = "mysql_small".to_owned();
        *mysql_small.name_mut() = "MySQL (Small)".to_owned();
        mysql.plans_mut().push(mysql_small);

        catalog.services_mut().push(mysql);


        let mut pgsql = model::Service::new();
        *pgsql.id_mut() = "pgsql".to_owned();
        *pgsql.name_mut() = "PostgreSQL".to_owned();

        let mut pgsql_free = model::ServicePlan::new();
        *pgsql_free.id_mut() = "pgsql_free".to_owned();
        *pgsql_free.name_mut() = "PostgreSQL (Free)".to_owned();
        pgsql.plans_mut().push(pgsql_free);

        let mut pgsql_small = model::ServicePlan::new();
        *pgsql_small.id_mut() = "pgsql_small".to_owned();
        *pgsql_small.name_mut() = "PostgreSQL (Small)".to_owned();
        pgsql.plans_mut().push(pgsql_small);

        catalog.services_mut().push(pgsql);


        catalog
    }



    fn check_catalog_provider(provider: &dyn CatalogProvider) {
        let catalog  = provider.get_catalog().expect("Error on retrieving catalog");
        let mut services = catalog.services().iter();


        if let Some(mysql) = services.next() {
            assert_eq!("mysql", mysql.id(), "mysql.id");
            assert_eq!("MySQL", mysql.name(), "mysql.name");

            let mut plans = mysql.plans().iter();

            if let Some(mysql_free) = plans.next() {
                assert_eq!("mysql_free", mysql_free.id(), "mysql.plans.free.id");
                assert_eq!("MySQL (Free)", mysql_free.name(), "mysql.plans.free.name");
            } else {
                panic!("Missing MySQL Free plan");
            }

            if let Some(mysql_small) = plans.next() {
                assert_eq!("mysql_small", mysql_small.id(), "mysql.plans.small.id");
                assert_eq!("MySQL (Small)", mysql_small.name(), "mysql.plans.small.name");
            }

            assert!(plans.next().is_none(), "mysql.plans.end");
        } else {
            panic!("Missing MySQL service");
        }


        if let Some(pgsql) = services.next() {
            assert_eq!("pgsql", pgsql.id(), "pgsql.id");
            assert_eq!("PostgreSQL", pgsql.name(), "pgsql.name");

            let mut plans = pgsql.plans().iter();

            if let Some(pgsql_free) = plans.next() {
                assert_eq!("pgsql_free", pgsql_free.id(), "pgsql.plans.free.id");
                assert_eq!("PostgreSQL (Free)", pgsql_free.name(), "pgsql.plans.free.name");
            } else {
                panic!("Missing PostgreSQL Free plan");
            }

            if let Some(pgsql_small) = plans.next() {
                assert_eq!("pgsql_small", pgsql_small.id(), "pgsql.plans.small.id");
                assert_eq!("PostgreSQL (Small)", pgsql_small.name(), "pgsql.plans.small.name");
            }

            assert!(plans.next().is_none(), "pgsql.plans.end");
        } else {
            panic!("Missing PostgreSQL service");
        }


        assert!(services.next().is_none(), "services.end");
    }

    #[test]
    fn catalog_provider_static() {
        let provider = SingleCatalogProvider::new(build_catalog());
        check_catalog_provider(&provider);
    }

    #[test]
    fn catalog_provider_dynamic_file_json() {
        let provider = JsonFileCatalogProvider::new("tests/default_catalog.json");
        check_catalog_provider(&provider);
    }

    #[test]
    fn catalog_provider_dynamic_file_json_missing() {
        let provider = JsonFileCatalogProvider::new("tests/missing_catalog.json");
        let error = provider.get_catalog().err().expect("catalog load MUST fail");
        let ioerror = error.downcast_ref::<std::io::Error>().expect("catalog load error must be an I/O one");
        assert_eq!(std::io::ErrorKind::NotFound, ioerror.kind());
    }

    #[test]
    fn catalog_provider_caching() {
        struct Counting<'a> {
            count: &'a std::cell::Cell<u32>,
        }
        impl<'a> Counting<'a> {
            fn new(count: &'a std::cell::Cell<u32>) -> Self {
                Counting {
                    count
                }
            }
        }
        impl<'a> CatalogProvider for Counting<'a> {
            fn get_catalog(&self) -> Result<std::borrow::Cow<model::Catalog>> {
                self.count.set(self.count.get() + 1);
                Ok(std::borrow::Cow::Owned(model::Catalog::new()))
            }
        }
        let counter = std::cell::Cell::default();

        let cache   = CachingCatalogProvider::new(Counting::new(&counter));
        assert_eq!(0, counter.get());

        assert!(cache.get_catalog().is_ok());
        assert_eq!(1, counter.get());

        assert!(cache.get_catalog().is_ok());
        assert_eq!(1, counter.get());
    }
}
