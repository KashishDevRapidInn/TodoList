use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;


mod todolist;
use todolist::services;

lazy_static! {
    
    static ref HOST: String = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    static ref PORT: String = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
}

struct AppState{
    entries: Mutex<Vec<TodoListEntry>>
}

#[derive(Serialize, Deserialize, Clone)]
struct TodoListEntry{
    id: i32,
    date: i64,
    title: String
}

#[get("/")]
async fn index()-> String{
    "Application Working".to_string()
}

#[actix_web::main]
async fn main()-> std::io::Result<()>{

    dotenv().ok(); // Load .env file

    let host = &*HOST;
    let port = &*PORT;


    let app_data= web::Data::new(AppState{
        entries: Mutex::new(vec![])
    });
    HttpServer::new(move||{
        App::new()
        .app_data(app_data.clone())
        .service(index)
        .configure(services::config)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}