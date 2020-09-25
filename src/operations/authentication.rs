use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/operations/schema.json",
    query_path = "src/operations/authentication.graphql"
)]
pub struct Register;
