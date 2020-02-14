Rust - Open Service Broker API
===



## Presentation

This projects aims to demonstrate [Open Service Broker](https://www.openservicebrokerapi.org/) implementation using Rust language.



## What is Open Service Broker API ?

On Cloud platform such as Cloud Foundry, Heroku, Kubernetes, ... it is very important to deploy applications. And it's also very important to integrate services such as database, a message-oriented middleware, etc. There are often call managed services and accessible through a service catalog.

The Open Service Broker API aims to provide an easy way to extend this service catalog. Whatever you're a service vendor wanting to have a Cloud offering or an engineering team wanting to provide enterprise services to development teams, you can expose your own catalog and make it "consumable" by development teams.

The Open Service Broker API has been first defined and used by Pivotal in its Cloud Foundry solution. Then the specification has been opened, so service offering provider can integrate with many Cloud solution. For example, [the Kubernetes Service Catalog ISG](https://svc-cat.io/) is responsible of integration with Kubernetes.

If you want more information, please visit: https://www.openservicebrokerapi.org/



## How to process ?

It currently exists some framework to help you develop an Open Service Broker without to deal with low level (HTTP-based API) consideration:

* [Spring Cloud Open Service Broker](https://spring.io/projects/spring-cloud-open-service-broker) (JVM)
* [brokerapi](https://github.com/pivotal-cf/brokerapi) (Go)
* [Open Service Broker API for .NET](https://github.com/AXOOM/OpenServiceBroker) (.Net)

So, idea is to provide a library crate handling HTTP API and delegating calls to some abstractions (i.e. `trait`).

Following Test-Driven Development, we will:

1. right some tests,
1. implements them,
1. optionally improves written code,
1. and going back to (1) until specification coverage is complete
