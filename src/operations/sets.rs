use graphql_client::GraphQLQuery;

type BigInt = i128;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/operations/schema.json",
    query_path = "src/operations/sets.graphql"
)]
pub struct UserSets;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/operations/schema.json",
    query_path = "src/operations/sets.graphql"
)]
pub struct CreateSet;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/operations/schema.json",
    query_path = "src/operations/sets.graphql"
)]
pub struct DeleteSet;
