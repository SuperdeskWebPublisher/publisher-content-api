#[derive(Queryable)]
// #[derive(GraphQLObject)]
// #[graphql(description="A humanoid creature in the Star Wars universe")]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub body: String,
}
