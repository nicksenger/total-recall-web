use graphql_client::GraphQLQuery;

type BigInt = u128;

macro_rules! generate_query {
    ($query:ident) => {
        #[derive(GraphQLQuery)]
        #[graphql(
            schema_path = "src/operations/schema.json",
            query_path = "src/operations/sets.graphql",
            response_derives = "Debug"
        )]
        pub struct $query;
    };
}
generate_query!(UserSets);
generate_query!(CreateSet);
generate_query!(DeleteSet);
