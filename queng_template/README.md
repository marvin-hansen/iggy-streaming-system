# Template services

This folder contains context generic microservice templates used throughout the project.

Context generic means, that the template is able to run any context that implement the
template trait bounds to provide said context as a microservice.

The rational for the template design pattern is to centralize code maintenance in one single crate that
is shared between all applicable microservices.

## Data service template

The IMS data integration service template is used to run all data services in the IMS data system.
The template specifies that the integration context used in the template must implement
the [ImsDataIntegration](../queng_traits/trait_data_integration/src/lib.rs) trait.

```rust
pub async fn start<Integration>(
    dbg: bool,
    exchange_id: ExchangeID,
    ims_integration: Integration,
) -> Result<(), Box<dyn std::error::Error>>
where
    Integration: ImsDataIntegration + Send + Sync + 'static,
{
 //...  
} 
```  

Taking the [Binance Spot integration](../queng_system_ims_data/binance/binance_spot/src/main.rs) as an example,
the template is used by creating a 'main.rs' file with the context shown below.

```rust
const DBG: bool = true;
const EXCHANGE_ID: ExchangeID = ExchangeID::BinanceSpot;
  
#[tokio::main]
async fn main() -> Result<(), Error> {
    ims_data_service::start(DBG, EXCHANGE_ID, ImsBinanceSpotDataIntegration::new())
        .await
        .expect("Failed to start Binance IMS Data service");

    Ok(())
}
```

Because the ImsBinanceSpotDataIntegration implements the ImsDataIntegration, the template service
will then provide the Binance Spot integration as a data microservice.