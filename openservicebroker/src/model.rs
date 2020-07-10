use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Catalog {
    services: Vec<Service>,
}

impl Catalog {
    pub fn new() -> Catalog {
        Catalog { services: vec![] }
    }

    pub fn services(&self) -> &Vec<Service> {
        &self.services
    }
    pub fn services_mut(&mut self) -> &mut Vec<Service> {
        &mut self.services
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    name: String,
    id: String,
    description: String,
    tags: Vec<String>,
    requires: Vec<String>,
    bindable: bool,
    instances_retrievable: Option<bool>,
    bindings_retrievable: Option<bool>,
    allow_context_updates: Option<bool>,
    metadata: HashMap<String, String>,
    // dashboard_client: Option<DashboardClient>,
    plan_updateable: Option<bool>,
    plans: Vec<ServicePlan>,
}

impl Service {
    pub fn new() -> Service {
        Service {
            name: String::new(),
            id: String::new(),
            description: String::new(),
            tags: Vec::new(),
            requires: Vec::new(),
            bindable: false,
            instances_retrievable: None,
            bindings_retrievable: None,
            allow_context_updates: None,
            metadata: HashMap::new(),
            plan_updateable: None,
            plans: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn id_mut(&mut self) -> &mut String {
        &mut self.id
    }

    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn description_mut(&mut self) -> &mut String {
        &mut self.description
    }

    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }
    pub fn tags_mut(&mut self) -> &mut Vec<String> {
        &mut self.tags
    }

    pub fn requires(&self) -> &Vec<String> {
        &self.requires
    }
    pub fn requires_mut(&mut self) -> &mut Vec<String> {
        &mut self.requires
    }

    pub fn bindable(&self) -> bool {
        self.bindable
    }
    pub fn bindable_mut(&mut self) -> &mut bool {
        &mut self.bindable
    }

    pub fn instances_retrievable(&self) -> Option<bool> {
        self.instances_retrievable
    }
    pub fn instances_retrievable_mut(&mut self) -> &mut Option<bool> {
        &mut self.instances_retrievable
    }

    pub fn bindings_retrievable(&self) -> Option<bool> {
        self.bindings_retrievable
    }
    pub fn bindings_retrievable_mut(&mut self) -> &mut Option<bool> {
        &mut self.bindings_retrievable
    }

    pub fn allow_context_updates(&self) -> Option<bool> {
        self.allow_context_updates
    }
    pub fn allow_context_updates_mut(&mut self) -> &mut Option<bool> {
        &mut self.allow_context_updates
    }

    pub fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }
    pub fn metadata_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.metadata
    }

    pub fn plan_updateable(&self) -> Option<bool> {
        self.plan_updateable
    }
    pub fn plan_updateable_mut(&mut self) -> &mut Option<bool> {
        &mut self.plan_updateable
    }

    pub fn plans(&self) -> &Vec<ServicePlan> {
        &self.plans
    }
    pub fn plans_mut(&mut self) -> &mut Vec<ServicePlan> {
        &mut self.plans
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicePlan {
    id: String,
    name: String,
    description: String,
    metadata: HashMap<String, String>,
    free: Option<bool>,
    bindable: Option<bool>,
    plan_updateable: Option<bool>,
    // schemas: Option<Schemas>,
    maximum_polling_duration: Option<u64>,
    // maintenance_info: Option<MaintenanceInfo>,
}

impl ServicePlan {
    pub fn new() -> ServicePlan {
        ServicePlan {
            id: String::new(),
            name: String::new(),
            description: String::new(),
            metadata: HashMap::new(),
            free: None,
            bindable: None,
            plan_updateable: None,
            maximum_polling_duration: None,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn id_mut(&mut self) -> &mut String {
        &mut self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn description_mut(&mut self) -> &mut String {
        &mut self.description
    }

    pub fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }
    pub fn metadata_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.metadata
    }

    pub fn free(&self) -> Option<bool> {
        self.free
    }
    pub fn free_mut(&mut self) -> &mut Option<bool> {
        &mut self.free
    }

    pub fn bindable(&self) -> Option<bool> {
        self.bindable
    }
    pub fn bindable_mut(&mut self) -> &mut Option<bool> {
        &mut self.bindable
    }

    pub fn plan_updateable(&self) -> Option<bool> {
        self.plan_updateable
    }
    pub fn plan_updateable_mut(&mut self) -> &mut Option<bool> {
        &mut self.plan_updateable
    }

    pub fn maximum_polling_duration(&self) -> Option<u64> {
        self.maximum_polling_duration
    }
    pub fn maximum_polling_duration_mut(&mut self) -> &mut Option<u64> {
        &mut self.maximum_polling_duration
    }
}

#[cfg(test)]
mod tests {
    mod catalog {
        use std::collections::HashMap;
        use super::super::{Catalog, Service, ServicePlan};

        #[test]
        fn catalog_new() {
            let catalog = Catalog::new();
            assert_eq!(0, catalog.services().len(), "services.len()");
        }

        #[test]
        fn catalog_update() {
            let mut catalog = Catalog::new();

            *catalog.services_mut() = vec![Service::new()];
            assert_eq!(1, catalog.services().len(), "[Set] services.len()");

            catalog.services_mut().push(Service::new());
            assert_eq!(2, catalog.services().len(), "[Push] services.len()");
        }

        #[test]
        fn service_new() {
            let service = Service::new();
            assert_eq!("", service.name(), "name");
            assert_eq!("", service.id(), "id");
            assert_eq!("", service.description(), "description");
            assert_eq!(0, service.tags().len(), "tags.len");
            assert_eq!(0, service.requires().len(), "requires.len");
            assert_eq!(false, service.bindable(), "bindable");
            assert_eq!(None, service.instances_retrievable(), "instances_retrievable");
            assert_eq!(None, service.bindings_retrievable(), "bindings_retrievable");
            assert_eq!(None, service.allow_context_updates(), "allow_context_updates");
            assert_eq!(0, service.metadata().len(), "metadata.len");
            assert_eq!(None, service.plan_updateable(), "plan_updateable");
            assert_eq!(0, service.plans().len(), "plans.len");
        }

        #[test]
        fn service_name() {
            let mut service = Service::new();

            *service.name_mut() = String::from("0");
            assert_eq!("0", service.name(), "[0]");

            service.name_mut().push_str("1");
            assert_eq!("01", service.name(), "[1]");

            assert_eq!("", service.id(), "id");
            assert_eq!("", service.description(), "description");
            assert_eq!(0, service.tags().len(), "tags.len");
            assert_eq!(0, service.requires().len(), "requires.len");
            assert_eq!(false, service.bindable(), "bindable");
            assert_eq!(None, service.instances_retrievable(), "instances_retrievable");
            assert_eq!(None, service.bindings_retrievable(), "bindings_retrievable");
            assert_eq!(None, service.allow_context_updates(), "allow_context_updates");
            assert_eq!(0, service.metadata().len(), "metadata.len");
            assert_eq!(None, service.plan_updateable(), "plan_updateable");
            assert_eq!(0, service.plans().len(), "plans.len");
        }

        #[test]
        fn service_id() {
            let mut service = Service::new();

            *service.id_mut() = String::from("0");
            assert_eq!("0", service.id(), "[0]");

            service.id_mut().push_str("1");
            assert_eq!("01", service.id(), "[1]");

            assert_eq!("", service.name(), "name");
            assert_eq!("", service.description(), "description");
            assert_eq!(0, service.tags().len(), "tags.len");
            assert_eq!(0, service.requires().len(), "requires.len");
            assert_eq!(false, service.bindable(), "bindable");
            assert_eq!(None, service.instances_retrievable(), "instances_retrievable");
            assert_eq!(None, service.bindings_retrievable(), "bindings_retrievable");
            assert_eq!(None, service.allow_context_updates(), "allow_context_updates");
            assert_eq!(0, service.metadata().len(), "metadata.len");
            assert_eq!(None, service.plan_updateable(), "plan_updateable");
            assert_eq!(0, service.plans().len(), "plans.len");
        }

        #[test]
        fn service_description() {
            let mut service = Service::new();

            *service.description_mut() = String::from("0");
            assert_eq!("0", service.description(), "[0]");

            service.description_mut().push_str("1");
            assert_eq!("01", service.description(), "[1]");

            assert_eq!("", service.name(), "name");
            assert_eq!("", service.id(), "id");
            assert_eq!(0, service.tags().len(), "tags.len");
            assert_eq!(0, service.requires().len(), "requires.len");
            assert_eq!(false, service.bindable(), "bindable");
            assert_eq!(None, service.instances_retrievable(), "instances_retrievable");
            assert_eq!(None, service.bindings_retrievable(), "bindings_retrievable");
            assert_eq!(None, service.allow_context_updates(), "allow_context_updates");
            assert_eq!(0, service.metadata().len(), "metadata.len");
            assert_eq!(None, service.plan_updateable(), "plan_updateable");
            assert_eq!(0, service.plans().len(), "plans.len");
        }

        #[test]
        fn service_tags() {
            let mut service = Service::new();

            *service.tags_mut() = vec![String::new()];
            assert_eq!(1, service.tags().len(), "[Set] tags.len()");

            service.tags_mut().push(String::new());
            assert_eq!(2, service.tags().len(), "[Push] tags.len()");

            assert_eq!("", service.name(), "name");
            assert_eq!("", service.id(), "id");
            assert_eq!("", service.description(), "description");
            assert_eq!(0, service.requires().len(), "requires.len");
            assert_eq!(false, service.bindable(), "bindable");
            assert_eq!(None, service.instances_retrievable(), "instances_retrievable");
            assert_eq!(None, service.bindings_retrievable(), "bindings_retrievable");
            assert_eq!(None, service.allow_context_updates(), "allow_context_updates");
            assert_eq!(0, service.metadata().len(), "metadata.len");
            assert_eq!(None, service.plan_updateable(), "plan_updateable");
            assert_eq!(0, service.plans().len(), "plans.len");
        }

        #[test]
        fn service_requires() {
            let mut service = Service::new();

            *service.requires_mut() = vec![String::new()];
            assert_eq!(1, service.requires().len(), "[Set] requires.len()");

            service.requires_mut().push(String::new());
            assert_eq!(2, service.requires().len(), "[Push] requires.len()");

            assert_eq!("", service.name(), "name");
            assert_eq!("", service.id(), "id");
            assert_eq!("", service.description(), "description");
            assert_eq!(0, service.tags().len(), "tags.len");
            assert_eq!(false, service.bindable(), "bindable");
            assert_eq!(None, service.instances_retrievable(), "instances_retrievable");
            assert_eq!(None, service.bindings_retrievable(), "bindings_retrievable");
            assert_eq!(None, service.allow_context_updates(), "allow_context_updates");
            assert_eq!(0, service.metadata().len(), "metadata.len");
            assert_eq!(None, service.plan_updateable(), "plan_updateable");
            assert_eq!(0, service.plans().len(), "plans.len");
        }

        #[test]
        fn service_bindable() {
            let mut service = Service::new();

            *service.bindable_mut() = true;
            assert_eq!(true, service.bindable(), "bindable");

            assert_eq!("", service.name(), "name");
            assert_eq!("", service.id(), "id");
            assert_eq!("", service.description(), "description");
            assert_eq!(0, service.tags().len(), "tags.len");
            assert_eq!(0, service.requires().len(), "requires.len");
            assert_eq!(None, service.instances_retrievable(), "instances_retrievable");
            assert_eq!(None, service.bindings_retrievable(), "bindings_retrievable");
            assert_eq!(None, service.allow_context_updates(), "allow_context_updates");
            assert_eq!(0, service.metadata().len(), "metadata.len");
            assert_eq!(None, service.plan_updateable(), "plan_updateable");
            assert_eq!(0, service.plans().len(), "plans.len");
        }

        #[test]
        fn service_instances_retrievable() {
            let mut service = Service::new();

            *service.instances_retrievable_mut() = Some(false);
            assert_eq!(Some(false), service.instances_retrievable(), "[false]");

            service.instances_retrievable_mut().replace(true);
            assert_eq!(Some(true), service.instances_retrievable(), "[true]");

            assert_eq!("", service.name(), "name");
            assert_eq!("", service.id(), "id");
            assert_eq!("", service.description(), "description");
            assert_eq!(0, service.tags().len(), "tags.len");
            assert_eq!(0, service.requires().len(), "requires.len");
            assert_eq!(false, service.bindable(), "bindable");
            assert_eq!(None, service.bindings_retrievable(), "bindings_retrievable");
            assert_eq!(None, service.allow_context_updates(), "allow_context_updates");
            assert_eq!(0, service.metadata().len(), "metadata.len");
            assert_eq!(None, service.plan_updateable(), "plan_updateable");
            assert_eq!(0, service.plans().len(), "plans.len");
        }

        #[test]
        fn service_bindings_retrievable() {
            let mut service = Service::new();

            *service.bindings_retrievable_mut() = Some(false);
            assert_eq!(Some(false), service.bindings_retrievable(), "[false]");

            service.bindings_retrievable_mut().replace(true);
            assert_eq!(Some(true), service.bindings_retrievable(), "[true]");

            assert_eq!("", service.name(), "name");
            assert_eq!("", service.id(), "id");
            assert_eq!("", service.description(), "description");
            assert_eq!(0, service.tags().len(), "tags.len");
            assert_eq!(0, service.requires().len(), "requires.len");
            assert_eq!(false, service.bindable(), "bindable");
            assert_eq!(None, service.instances_retrievable(), "instances_retrievable");
            assert_eq!(None, service.allow_context_updates(), "allow_context_updates");
            assert_eq!(0, service.metadata().len(), "metadata.len");
            assert_eq!(None, service.plan_updateable(), "plan_updateable");
            assert_eq!(0, service.plans().len(), "plans.len");
        }

        #[test]
        fn service_allow_context_updates() {
            let mut service = Service::new();

            *service.allow_context_updates_mut() = Some(false);
            assert_eq!(Some(false), service.allow_context_updates(), "[false]");

            service.allow_context_updates_mut().replace(true);
            assert_eq!(Some(true), service.allow_context_updates(), "[true]");

            assert_eq!("", service.name(), "name");
            assert_eq!("", service.id(), "id");
            assert_eq!("", service.description(), "description");
            assert_eq!(0, service.tags().len(), "tags.len");
            assert_eq!(0, service.requires().len(), "requires.len");
            assert_eq!(false, service.bindable(), "bindable");
            assert_eq!(None, service.instances_retrievable(), "instances_retrievable");
            assert_eq!(None, service.bindings_retrievable(), "bindings_retrievable");
            assert_eq!(0, service.metadata().len(), "metadata.len");
            assert_eq!(None, service.plan_updateable(), "plan_updateable");
            assert_eq!(0, service.plans().len(), "plans.len");
        }

        #[test]
        fn service_metadata() {
            let mut service = Service::new();

            let zero = "0".to_owned();
            let un   = "1".to_owned();

            let mut metadata: HashMap<String, String> = HashMap::new();
            metadata.insert(zero.clone(), zero.clone());
            *service.metadata_mut() = metadata;
            assert_eq!(1, service.metadata().len(), "[Set] metadata.len()");

            service.metadata_mut().insert(un.clone(), un.clone());
            assert_eq!(2, service.metadata().len(), "[Insert] metadata.len()");

            assert_eq!("", service.name(), "name");
            assert_eq!("", service.id(), "id");
            assert_eq!("", service.description(), "description");
            assert_eq!(0, service.tags().len(), "tags.len");
            assert_eq!(0, service.requires().len(), "requires.len");
            assert_eq!(false, service.bindable(), "bindable");
            assert_eq!(None, service.instances_retrievable(), "instances_retrievable");
            assert_eq!(None, service.bindings_retrievable(), "bindings_retrievable");
            assert_eq!(None, service.allow_context_updates(), "allow_context_updates");
            assert_eq!(None, service.plan_updateable(), "plan_updateable");
            assert_eq!(0, service.plans().len(), "plans.len");
        }

        #[test]
        fn service_plan_updateable() {
            let mut service = Service::new();

            *service.plan_updateable_mut() = Some(false);
            assert_eq!(Some(false), service.plan_updateable(), "[false]");

            service.plan_updateable_mut().replace(true);
            assert_eq!(Some(true), service.plan_updateable(), "[true]");

            assert_eq!("", service.name(), "name");
            assert_eq!("", service.id(), "id");
            assert_eq!("", service.description(), "description");
            assert_eq!(0, service.tags().len(), "tags.len");
            assert_eq!(0, service.requires().len(), "requires.len");
            assert_eq!(false, service.bindable(), "bindable");
            assert_eq!(None, service.instances_retrievable(), "instances_retrievable");
            assert_eq!(None, service.bindings_retrievable(), "bindings_retrievable");
            assert_eq!(None, service.allow_context_updates(), "allow_context_updates");
            assert_eq!(0, service.metadata().len(), "metadata.len");
            assert_eq!(0, service.plans().len(), "plans.len");
        }

        #[test]
        fn service_plans() {
            let mut service = Service::new();

            *service.plans_mut() = vec![ServicePlan::new()];
            assert_eq!(1, service.plans().len(), "[Set] requires.len()");

            service.plans_mut().push(ServicePlan::new());
            assert_eq!(2, service.plans().len(), "[Push] requires.len()");

            assert_eq!("", service.name(), "name");
            assert_eq!("", service.id(), "id");
            assert_eq!("", service.description(), "description");
            assert_eq!(0, service.tags().len(), "tags.len");
            assert_eq!(0, service.requires().len(), "requires.len");
            assert_eq!(false, service.bindable(), "bindable");
            assert_eq!(None, service.instances_retrievable(), "instances_retrievable");
            assert_eq!(None, service.bindings_retrievable(), "bindings_retrievable");
            assert_eq!(None, service.allow_context_updates(), "allow_context_updates");
            assert_eq!(0, service.metadata().len(), "metadata.len");
            assert_eq!(None, service.plan_updateable(), "plan_updateable");
        }

        #[test]
        fn serviceplan_new() {
            let plan = ServicePlan::new();

            assert_eq!("", plan.id(), "id");
            assert_eq!("", plan.name(), "name");
            assert_eq!("", plan.description(), "description");
            assert_eq!(0, plan.metadata().len(), "metadata.len");
            assert_eq!(None, plan.free(), "free");
            assert_eq!(None, plan.bindable(), "bindable");
            assert_eq!(None, plan.maximum_polling_duration(), "maximum_polling_duration");
        }

        #[test]
        fn serviceplan_id() {
            let mut plan = ServicePlan::new();

            *plan.id_mut() = String::from("0");
            assert_eq!("0", plan.id(), "[0]");

            plan.id_mut().push_str("1");
            assert_eq!("01", plan.id(), "[1]");

            assert_eq!("", plan.name(), "name");
            assert_eq!("", plan.description(), "description");
            assert_eq!(0, plan.metadata().len(), "metadata.len");
            assert_eq!(None, plan.free(), "free");
            assert_eq!(None, plan.bindable(), "bindable");
            assert_eq!(None, plan.maximum_polling_duration(), "maximum_polling_duration");
        }

        #[test]
        fn serviceplan_name() {
            let mut plan = ServicePlan::new();

            *plan.name_mut() = String::from("0");
            assert_eq!("0", plan.name(), "[0]");

            plan.name_mut().push_str("1");
            assert_eq!("01", plan.name(), "[1]");

            assert_eq!("", plan.id(), "id");
            assert_eq!("", plan.description(), "description");
            assert_eq!(0, plan.metadata().len(), "metadata.len");
            assert_eq!(None, plan.free(), "free");
            assert_eq!(None, plan.bindable(), "bindable");
            assert_eq!(None, plan.maximum_polling_duration(), "maximum_polling_duration");
        }

        #[test]
        fn serviceplan_description() {
            let mut plan = ServicePlan::new();

            *plan.description_mut() = String::from("0");
            assert_eq!("0", plan.description(), "[0]");

            plan.description_mut().push_str("1");
            assert_eq!("01", plan.description(), "[1]");

            assert_eq!("", plan.id(), "id");
            assert_eq!("", plan.name(), "name");
            assert_eq!(0, plan.metadata().len(), "metadata.len");
            assert_eq!(None, plan.free(), "free");
            assert_eq!(None, plan.bindable(), "bindable");
            assert_eq!(None, plan.maximum_polling_duration(), "maximum_polling_duration");
        }

        #[test]
        fn serviceplan_metadata() {
            let mut plan = ServicePlan::new();

            let zero = "0".to_owned();
            let un   = "1".to_owned();

            let mut metadata: HashMap<String, String> = HashMap::new();
            metadata.insert(zero.clone(), zero.clone());
            *plan.metadata_mut() = metadata;
            assert_eq!(1, plan.metadata().len(), "[Set] metadata.len()");

            plan.metadata_mut().insert(un.clone(), un.clone());
            assert_eq!(2, plan.metadata().len(), "[Insert] metadata.len()");

            assert_eq!("", plan.id(), "id");
            assert_eq!("", plan.name(), "name");
            assert_eq!("", plan.description(), "description");
            assert_eq!(None, plan.free(), "free");
            assert_eq!(None, plan.bindable(), "bindable");
            assert_eq!(None, plan.maximum_polling_duration(), "maximum_polling_duration");
        }

        #[test]
        fn serviceplan_free() {
            let mut plan = ServicePlan::new();

            *plan.free_mut() = Some(false);
            assert_eq!(Some(false), plan.free(), "[false]");

            plan.free_mut().replace(true);
            assert_eq!(Some(true), plan.free(), "[true]");

            assert_eq!("", plan.id(), "id");
            assert_eq!("", plan.name(), "name");
            assert_eq!("", plan.description(), "description");
            assert_eq!(0, plan.metadata().len(), "metadata.len");
            assert_eq!(None, plan.bindable(), "bindable");
            assert_eq!(None, plan.maximum_polling_duration(), "maximum_polling_duration");
        }

        #[test]
        fn serviceplan_bindable() {
            let mut plan = ServicePlan::new();

            *plan.bindable_mut() = Some(false);
            assert_eq!(Some(false), plan.bindable(), "[false]");

            plan.bindable_mut().replace(true);
            assert_eq!(Some(true), plan.bindable(), "[true]");

            assert_eq!("", plan.id(), "id");
            assert_eq!("", plan.name(), "name");
            assert_eq!("", plan.description(), "description");
            assert_eq!(0, plan.metadata().len(), "metadata.len");
            assert_eq!(None, plan.free(), "free");
            assert_eq!(None, plan.maximum_polling_duration(), "maximum_polling_duration");
        }

        #[test]
        fn serviceplan_maximum_polling_duration() {
            let mut plan = ServicePlan::new();

            *plan.maximum_polling_duration_mut() = Some(0);
            assert_eq!(Some(0), plan.maximum_polling_duration(), "[0]");

            plan.maximum_polling_duration_mut().replace(1);
            assert_eq!(Some(1), plan.maximum_polling_duration(), "[1]");

            assert_eq!("", plan.id(), "id");
            assert_eq!("", plan.name(), "name");
            assert_eq!("", plan.description(), "description");
            assert_eq!(0, plan.metadata().len(), "metadata.len");
            assert_eq!(None, plan.free(), "free");
            assert_eq!(None, plan.bindable(), "bindable");
        }
    }
}
