Rust - Open Service Broker API
===



## Presentation

This projects aims to demonstrate [Open Service Broker](https://www.openservicebrokerapi.org/) implementation using Rust language.



## What is Open Service Broker API ?

On Cloud platform such as Cloud Foundry, Heroku, Kubernetes, ... it is very important to deploy applications. And it's also very important to integrate services such as database, a message-oriented middleware, etc. There are often call managed services and accessible through a service catalog.

The Open Service Broker API aims to provide an easy way to extend this service catalog. Whatever you're a service vendor wanting to have a Cloud offering or an engineering team wanting to provide enterprise services to development teams, you can expose your own catalog and make it "consumable" by development teams.

The Open Service Broker API has been first defined and used by Pivotal in its Cloud Foundry solution. Then the specification has been opened, so service offering provider can integrate with many Cloud solution. For example, [the Kubernetes Service Catalog ISG](https://svc-cat.io/) is responsible of integration with Kubernetes.

If you want more information, please visit: https://www.openservicebrokerapi.org/
