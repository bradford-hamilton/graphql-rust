use juniper::FieldResult;
use juniper::RootNode;

#[derive(GraphQLEnum)]
enum Hobby {
    Coding,
    VideoGames,
    Racing,
}

#[derive(GraphQLObject)]
#[graphql(description = "Just a regular User with some hobbies!")]
struct UserType {
    id: String,
    name: String,
    favorite_hobby: Vec<Hobby>,
    job: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Just a regular User with some hobbies!")]
struct UserInput {
    name: String,
    favorite_hobby: Vec<Hobby>,
    job: String,
}

pub struct QueryRoot;

graphql_object!(QueryRoot: () |&self| {
    field User(&executor, id: String) -> FieldResult<UserType> {
        Ok(UserType{
            id: "123".to_owned(),
            name: "Luke".to_owned(),
            favorite_hobby: vec![Hobby::Racing],
            job: "Software Developer".to_owned(),
        })
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: () |&self| {
    field createUser(&executor, new_user: UserInput) -> FieldResult<UserType> {
        Ok(UserType{
            id: "123".to_owned(),
            name: new_user.name,
            favorite_hobby: new_user.favorite_hobby,
            job: new_user.job,
        })
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}