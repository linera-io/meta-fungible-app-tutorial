#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::MetaFungible;
use async_graphql::{EmptySubscription, Schema};
use linera_sdk::graphql::GraphQLMutationRoot;
use linera_sdk::{base::WithServiceAbi, Service, ServiceRuntime, ViewStateStorage};
use meta_fungible::Operation;
use std::sync::Arc;
use thiserror::Error;

#[derive(Clone)]
pub struct MetaFungibleService {
    state: Arc<MetaFungible>,
    _runtime: Arc<ServiceRuntime<Self>>,
}

linera_sdk::service!(MetaFungibleService);

impl WithServiceAbi for MetaFungibleService {
    type Abi = meta_fungible::MetaFungibleAbi;
}

impl Service for MetaFungibleService {
    type Error = ServiceError;
    type Storage = ViewStateStorage<Self>;
    type State = MetaFungible;

    async fn new(state: Self::State, runtime: ServiceRuntime<Self>) -> Result<Self, Self::Error> {
        Ok(MetaFungibleService {
            state: Arc::new(state),
            _runtime: Arc::new(runtime),
        })
    }

    async fn handle_query(&self, query: Self::Query) -> Result<Self::QueryResponse, Self::Error> {
        let schema = Schema::new(
            self.state.clone(),
            Operation::mutation_root(),
            EmptySubscription,
        );
        let response = schema.execute(query).await;
        Ok(response)
    }
}

/// An error that can occur while querying the service.
#[derive(Debug, Error)]
pub enum ServiceError {
    /// Query not supported by the application.
    #[error("Queries not supported by application")]
    QueriesNotSupported,

    /// Invalid query argument; could not deserialize request.
    #[error("Invalid query argument; could not deserialize request")]
    InvalidQuery(#[from] serde_json::Error),
    // Add error variants here.
}
