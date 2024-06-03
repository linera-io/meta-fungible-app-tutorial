use async_graphql::{Request, Response};
use fungible::Account;
use linera_sdk::base::{Amount, ContractAbi, Owner, ServiceAbi};
use linera_sdk::graphql::GraphQLMutationRoot;
use serde::{Deserialize, Serialize};

pub struct MetaFungibleAbi;

impl ContractAbi for MetaFungibleAbi {
    type Operation = Operation;
    type Response = ();
}

impl ServiceAbi for MetaFungibleAbi {
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    Transfer {
        owner: Owner,
        amount: Amount,
        target_account: Account,
    },
}
