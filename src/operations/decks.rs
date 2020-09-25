use graphql_client::GraphQLQuery;

type BigInt = u128;

macro_rules! generate_query {
    ($query:ident) => {
        #[derive(GraphQLQuery)]
        #[graphql(
            schema_path = "src/operations/schema.json",
            query_path = "src/operations/decks.graphql",
            response_derives = "Debug"
        )]
        pub struct $query;
    };
}
generate_query!(LanguageList);
generate_query!(UserDecks);
generate_query!(CreateDeck);
generate_query!(DeleteDeck);
