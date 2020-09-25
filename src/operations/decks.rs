use graphql_client::GraphQLQuery;

type BigInt = i128;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/operations/schema.json",
    query_path = "src/operations/decks.graphql"
)]
pub struct LanguageList;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/operations/schema.json",
    query_path = "src/operations/decks.graphql"
)]
pub struct UserDecks;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/operations/schema.json",
    query_path = "src/operations/decks.graphql"
)]
pub struct CreateDeck;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/operations/schema.json",
    query_path = "src/operations/decks.graphql"
)]
pub struct DeleteDeck;
