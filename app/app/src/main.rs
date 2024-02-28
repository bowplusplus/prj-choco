use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use mongodb::{Client, options::ClientOptions};
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use futures::stream::StreamExt;

struct AppState {
    client: Mutex<Client>,
}

#[derive(Serialize, Deserialize)]
struct Team {
    name: String,
}

// 非同期関数内で非同期ストリームを処理
async fn get_teams(data: web::Data<AppState>) -> impl Responder {
    let team_collection = data.client.lock().unwrap().database("test").collection::<Team>("teams");
    let mut cursor = team_collection.find(None, None).await.unwrap();

    let mut teams = Vec::new();
    while let Some(team) = cursor.next().await {
        match team {
            Ok(doc) => teams.push(doc),
            Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        }
    }

    HttpResponse::Ok().json(teams)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client_options = ClientOptions::parse("mongodb://root:example@mongodb:27017/").await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                client: Mutex::new(client.clone()),
            }))
            .route("/api/v1/teams", web::get().to(get_teams))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
