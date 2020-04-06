use juniper::{
    graphql_value, FieldError, FieldResult, GraphQLInputObject, GraphQLObject, RootNode,
};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref POSTS: Mutex<HashMap<i32, Post>> = Mutex::new(HashMap::new());
    static ref CURRENT_ID: Mutex<u32> = Mutex::new(0);
}

#[derive(Clone, GraphQLObject)]
#[graphql(description = "A post representation")]
struct Post {
    id: i32,
    title: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A post to create")]
struct CreatePost {
    title: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A post to update")]
struct UpdatePost {
    id: i32,
    title: String,
}

pub struct Query;

#[juniper::object]
impl Query {
    fn get_post(id: i32) -> FieldResult<Post> {
        let posts = POSTS.lock().unwrap();
        let post = posts.get(&id);
        if post.is_none() {
            return Err(FieldError::new(
                "Could not find a post",
                graphql_value!({ "internal_error": "Post not found" }),
            ));
        }

        Ok(post.unwrap().clone())
    }

    fn all_posts() -> FieldResult<Vec<Post>> {
        let posts = POSTS.lock().unwrap();
        let all_posts = posts
            .iter()
            .map(|(_, post)| post.clone())
            .collect::<Vec<Post>>();

        Ok(all_posts)
    }
}

pub struct Mutation;

#[juniper::object]
impl Mutation {
    fn create_post(input: CreatePost) -> FieldResult<Post> {
        let mut posts = POSTS.lock().unwrap();
        let mut id = CURRENT_ID.lock().unwrap();
        let new_id = *id as i32;
        let new_post = Post {
            id: new_id,
            title: input.title,
        };
        posts.insert(new_id, new_post.clone());
        *id += 1;

        Ok(new_post)
    }

    fn update_post(input: UpdatePost) -> FieldResult<Post> {
        let mut posts = POSTS.lock().unwrap();
        let post = posts.get(&input.id);
        if post.is_none() {
            return Err(FieldError::new(
                "Could not find a post",
                graphql_value!({ "internal_error": "Post not found" }),
            ));
        }

        let id = input.id;
        posts.entry(id).and_modify(|post| post.title = input.title);

        Ok(posts.get(&id).unwrap().clone())
    }

    fn delete_post(id: i32) -> FieldResult<Post> {
        let mut posts = POSTS.lock().unwrap();
        let post = posts.remove(&id);
        if post.is_none() {
            return Err(FieldError::new(
                "Could not find a post",
                graphql_value!({ "internal_error": "Post not found" }),
            ));
        }

        Ok(post.unwrap())
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
