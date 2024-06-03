#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::MetaFungible;
use async_graphql::{EmptySubscription, Schema};
use linera_sdk::graphql::GraphQLMutationRoot;
use linera_sdk::views::{View, ViewStorageContext};
use linera_sdk::{base::WithServiceAbi, Service, ServiceRuntime};
use meta_fungible::Operation;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct MetaFungibleService {
    state: Arc<MetaFungible>,
    #[allow(unused)]
    runtime: Arc<Mutex<ServiceRuntime<Self>>>,
}

linera_sdk::service!(MetaFungibleService);

impl WithServiceAbi for MetaFungibleService {
    type Abi = meta_fungible::MetaFungibleAbi;
}

impl Service for MetaFungibleService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = MetaFungible::load(ViewStorageContext::from(runtime.key_value_store()))
            .await
            .expect("Failed to load state");
        MetaFungibleService {
            state: Arc::new(state),
            runtime: Arc::new(Mutex::new(runtime)),
        }
    }

    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
        let schema = Schema::new(
            self.state.clone(),
            Operation::mutation_root(),
            EmptySubscription,
        );
        schema.execute(query).await
    }
}
