## Let's design catalog

The Open Service Broker API is based around three major concepts: service, instance and binding. Listing provided services is made through `/v2/catalog` endpoint. Let's first design ~~entities~~ structs ([spec](https://github.com/openservicebrokerapi/servicebroker/blob/v2.15/spec.md#service-offering-object)):

```rust
struct Catalog {
    services: Vec<Service>,
}

struct Service {
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

struct ServicePlan {
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
```

_Notes:_

* Accessor and mutator aren't shown there. They can be added as structs are coded, but can also be added during refactoring steps.
* _[Service metadata](https://github.com/openservicebrokerapi/servicebroker/blob/v2.15/profile.md#service-metadata) is supposed to be free-form object but to keep it simple, it's implemented as simple key-value string pairs._
* _[`DashboardClient`](https://github.com/openservicebrokerapi/servicebroker/blob/v2.15/profile.md#dashboard-client-object) is not implemented._
* _[`Schemas`](https://github.com/openservicebrokerapi/servicebroker/blob/v2.15/spec.md#schemas-object) is not implemented._
* _[`MaintenanceInfo`](https://github.com/openservicebrokerapi/servicebroker/blob/v2.15/spec.md#maintenance-info-object) is not implemented._
* _DON'T FORGET TO RE-RUN TEST !_

OK, that's a nice struct design but don't forget JSON support ! As previously, just add `#[derive(Serialize, Deserialize)]` from [Serde](https://serde.rs/) to each struct.

In order to make things cleaner, let's move all those structs into a `model` sub-module.
