use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::extensions::ApolloTracing;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};
use std::env;

mod application;
mod db;
mod domain;
mod persistance;
mod query;
mod state;

use query::QueryRoot;
use state::State;

type CultSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

async fn index(schema: web::Data<CultSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
        )))
}

#[actix_web::main]
async fn main() -> async_graphql::Result<()> {
    dotenv::dotenv().ok();
    let db_connection = db::db_connection().await?;
    let state = State::new(db_connection);
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(state)
        .extension(ApolloTracing)
        .finish();

    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_owned());
    let address = env::var("ADDRESS").unwrap_or_else(|_| "localhost".to_owned());

    let listen_addr = format!("{}:{}", address, port);
    println!("Playground: {}", listen_addr);

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/graphql").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind(listen_addr)?
    .run()
    .await?;

    Ok(())
}
