#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use fungible::{FungibleAbi, Message};
use linera_sdk::base::ApplicationId;
use linera_sdk::views::{RootView, View, ViewStorageContext};
use linera_sdk::{base::WithContractAbi, Contract, ContractRuntime};
use meta_fungible::Operation;

use self::state::MetaFungible;

pub struct MetaFungibleContract {
    state: MetaFungible,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(MetaFungibleContract);

impl WithContractAbi for MetaFungibleContract {
    type Abi = meta_fungible::MetaFungibleAbi;
}

impl MetaFungibleContract {
    fn fungible_id(&mut self) -> ApplicationId<fungible::FungibleAbi> {
        self.runtime.application_parameters()
    }
}

impl Contract for MetaFungibleContract {
    type Parameters = ApplicationId<FungibleAbi>;
    type InstantiationArgument = ();
    type Message = Message;

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = MetaFungible::load(ViewStorageContext::from(runtime.key_value_store()))
            .await
            .expect("Failed to load state");
        MetaFungibleContract { state, runtime }
    }

    async fn instantiate(&mut self, _: Self::InstantiationArgument) {}

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        match operation {
            Operation::Transfer {
                owner,
                amount,
                target_account,
            } => {
                let fungible_id = self.fungible_id();
                let operation = fungible::Operation::Transfer {
                    owner,
                    amount,
                    target_account,
                };
                self.runtime.call_application(true, fungible_id, &operation);
            }
        }
    }

    async fn execute_message(&mut self, _message: Self::Message) {}

    async fn store(mut self) {
        self.state.save().await.expect("Failed to persist state");
    }
}
