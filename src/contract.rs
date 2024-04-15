#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use async_trait::async_trait;
use linera_sdk::base::ApplicationId;
use linera_sdk::{base::WithContractAbi, Contract, ContractRuntime, ViewStateStorage};
use meta_fungible::Operation;
use thiserror::Error;

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

#[async_trait]
impl Contract for MetaFungibleContract {
    type Error = ContractError;
    type Storage = ViewStateStorage<Self>;
    type State = MetaFungible;
    type Message = ();

    async fn new(state: Self::State, runtime: ContractRuntime<Self>) -> Result<Self, Self::Error> {
        Ok(MetaFungibleContract { state, runtime })
    }

    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.state
    }

    async fn initialize(
        &mut self,
        _argument: Self::InitializationArgument,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn execute_operation(
        &mut self,
        operation: Self::Operation,
    ) -> Result<Self::Response, Self::Error> {
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
                Ok(())
            }
        }
    }

    async fn execute_message(&mut self, _message: Self::Message) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// An error that can occur during the contract execution.
#[derive(Debug, Error)]
pub enum ContractError {
    /// Failed to deserialize BCS bytes
    #[error("Failed to deserialize BCS bytes")]
    BcsError(#[from] bcs::Error),

    /// Failed to deserialize JSON string
    #[error("Failed to deserialize JSON string")]
    JsonError(#[from] serde_json::Error),
    // Add more error variants here.
}
