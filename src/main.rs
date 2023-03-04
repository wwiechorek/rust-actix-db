use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sqlx::mysql::{MySqlPoolOptions};
use sqlx::Pool;
use sqlx::MySql;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
}

#[get("/")]
async fn hello(
    data: web::Data<AppState>,
) -> impl Responder {
    let result = sqlx::query_as::<_, Item>("SELECT * FROM test")
        .fetch_all(&data.db)
        .await
        .unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": result.len(),
        "items": result
    });
    HttpResponse::Ok().json(json_response)
}

pub struct AppState {
    db: Pool<MySql>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = match MySqlPoolOptions::new()
        .connect("mysql://root:my_secret_password@localhost/app?ssl-mode=Disabled")
        .await
        {
            Ok(pool) => {
                println!("✅Connection to the database is successful!");
                pool
            }
            Err(err) => {
                println!("🔥 Failed to connect to the database: {:?}", err);
                std::process::exit(1);
            }
        };

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}