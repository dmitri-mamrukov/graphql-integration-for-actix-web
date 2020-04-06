# graphql-integration-for-actix-web
GraphQL integration for Actix Web

## Running

1. Start the GraphQL server via `cargo run`

2. `http://localhost:1978/graphiql`

3. In the GraphiQL interface, enter the following queries:

`query { allPosts { id title } }`

`query { getPost(id: 0) { id title } }`

`mutation { createPost(input: { title: "testTitle" }) }`

`query { getPost(id: 0) { id title } }`

`query { allPosts { id title } }`

`mutation { updatePost(input: { id: 0, title: "testTitleNew" }) }`

`query { getPost(id: 0) { id title } }`

`query { allPosts { id title } }`

`mutation { deletePost(id: 0) }`

`query { getPost(id: 0) { id title } }`

`query { allPosts { id title } }`

## Docker

1. `docker build --tag crud-server-image .`

2. `docker run -it --expose 1978 --publish 1978:1978 --rm --name crud-server-image crud-server-image`

Useful commands:

`docker stop $(docker ps --all --quiet)`

`docker inspect crud-server-image`
