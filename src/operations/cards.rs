use graphql_client::GraphQLQuery;

type BigInt = i128;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/operations/schema.json",
    query_path = "src/operations/cards.graphql"
)]
pub struct DeckCards;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/operations/schema.json",
    query_path = "src/operations/cards.graphql"
)]
pub struct CreateCard;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/operations/schema.json",
    query_path = "src/operations/cards.graphql"
)]
pub struct DeleteCard;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/operations/schema.json",
    query_path = "src/operations/cards.graphql"
)]
pub struct EditCardLink;
