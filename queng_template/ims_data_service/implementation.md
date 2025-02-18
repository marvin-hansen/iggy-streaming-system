# Data service template

## Overview

This service template is used to run all data integrations as a standalone IMS microservice.

## Concept

The core concept is simple: This ims_data_service is generic over the `ImsDataIntegration` trait
and thus takes any implementation of the the `ImsDataIntegration` trait as input and plugs it into the microservice
template so that it can be started from a regular Rust main function.

A standalone data integration implements the `ImsDataIntegration` trait
and through that provides various data streams from the integrated data source. For example, the
`binance_spot_data_integration` provides a stream of binance spot data by implementing the `ImsDataIntegration`
for the binance spot market.

Then, this microservice template implements all data streaming related functionality using the functionality
from the `ImsDataIntegration` trait.

In the main instance, then a specific data integration instance is created and plugged into the microservice template
to then start the microservice.

Lastly, the  `ImsDataClient` then connects to the microservice to start data streams and to process recieved data
by using an an implementation of the `MessageConsumer` trait that takes a `ReceivedMessage` from the data stream as
input and then processes it according to the custom implementation.

## Context

* `ImsDataIntegration` trait to provide a data stream integration for a specific data source
* `queng_integration/data` package that contains implementations of the `ImsDataIntegration` trait for various data
  sources
* `queng_system_ims_data` package that contains data source specific IMS data integration microservice based on the
  template.
* `ImsDataClient` client to interact with the IMS data microservice
* `MessageConsumer`(1) trait to process messages from the data stream in the `ImsDataClient`

(1) Note, the `MessageConsumer` trait resides in the external iggy crate in absence of an internal trait alias.

## Functional Requirements

### Client handling:

**ClientLogin**

**ClientLogout**

### Data stream handling:

**StartData**

**StopData**

**StopAllData**

## Non-Functional Requirements

### Performance

### Reliability

### Reliability
