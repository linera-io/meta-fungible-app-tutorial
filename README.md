# Meta-Fungible Application

## Getting Started

To get started, first create deploy the `fungible` application example by following
the application's [README](https://github.com/linera-io/fungible-app-tutorial/blob/main/README.md).

After deploying your `fungible` application, store your fungible application's ID as it will be used
to deploy the `meta-fungible` application.

## Deploying the application

The `meta-fungible` application works by calling the `fungible` application and invoking operations on it.

To deploy the `meta-fungible` application, use `fungible` application's ID (assuming it is stored
at `$FUNGIBLE_APP_ID`).

```bash
linera project publish-and-create \
    --required-application-ids $FUNGIBLE_APP_ID \
    --json-parameters "\"$FUNGIBLE_APP_ID\""
```

The required JSON parameters are used to reference the specific instance of the `fungible` application and the
`required-application-ids` are used to tell the client which applications we depend on.

## Running the application

To run the application, start a linera-service:

```bash
linera service
```

And navigate to `localhost:8080` starting the GraphQL IDE. At this point listing the applications on your default chain
should result in a list containing the `fungible` and `metafungible` application.

Open the link for the `meta-fungible` application and using the GraphQL call the transfer field to send tokens to a
desired account.

```gql
mutation {
    transfer(...) 
}
```

Then, navigating to the `fungible` application GraphQL interface, the updated balances can be inspected in the
`accounts` field.