use linera_sdk::views::{linera_views, RegisterView, RootView, ViewStorageContext};

#[derive(RootView, async_graphql::SimpleObject)]
#[view(context = "ViewStorageContext")]
pub struct MetaFungible {
    pub value: RegisterView<u64>,
    // Add fields here.
}
