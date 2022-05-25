use actix_web::{
    error, get,
    http::{self, StatusCode},
    middleware::Logger,
    post, web,
    web::{Data, Payload},
    App, Error, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder,
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::{self, tls::NoTlsStream, Client, Connection, NoTls, Socket};

#[derive(Serialize, Deserialize, Debug)]
pub struct AddCity {
    department_code: String,
    insee_code: String,
    zip_code: String,
    name: String,
    lat: f64,
    lon: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct City {
    id: i32,
    department_code: String,
    insee_code: String,
    zip_code: String,
    name: String,
    lat: f64,
    lon: f64,
}

#[post("/")]
async fn write_to_db(
    mut payload: web::Payload,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > 999999 {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    let obj = serde_json::from_slice::<AddCity>(&body)?;
    println!("{:?}", obj);
    let query = format!("INSERT INTO city (department_code, insee_code, zip_code, name, lat, lon) VALUES ('{}', '{}', '{}', '{}', {}, {})", obj.department_code, obj.insee_code, obj.zip_code, obj.name, obj.lat, obj.lon);
    &data
        .client
        .clone()
        .lock()
        .await
        .query(
            query.as_str(),
            &[
                &obj.department_code,
                &obj.insee_code,
                &obj.zip_code,
                &obj.name,
                &obj.lat,
                &obj.lon,
            ],
        )
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json(obj)) // <- send response
}

#[get("/")]
async fn get_cities(data: web::Data<AppState>) -> impl Responder {
    let query = "SELECT * FROM city";
    let rows = &data
        .client
        .clone()
        .lock()
        .await
        .query(query, &[])
        .await
        .unwrap();
    let mut cities = Vec::new();
    for row in rows {
        let city: City = City {
            id: row.get("id"),
            department_code: row.get("department_code"),
            insee_code: row.get("insee_code"),
            zip_code: row.get("zip_code"),
            name: row.get("name"),
            lat: row.get("lat"),
            lon: row.get("lon"),
        };
        cities.push(city);
    }
    HttpResponse::Ok().json(cities)
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::NoContent()
}

struct AppState {
    client: Arc<Mutex<Client>>,
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let api_addr = match std::env::var("CITY_API_ADDR") {
        Ok(val) => val,
        Err(_) => "0.0.0.0".to_string(),
    };
    let api_port = match std::env::var("CITY_API_PORT") {
        Ok(val) => val,
        Err(_) => "2022".to_string(),
    };

    let db_url = std::env::var("CITY_DB_URL").expect("CITY_DB_URL must be set");
    let db_user = std::env::var("CITY_DB_USER").expect("CITY_DB_USER must be set");
    let db_password = std::env::var("CITY_DB_PASSWORD").expect("CITY_DB_PASSWORD must be set");
    let db_url = format!("postgres://{}:{}@{}", db_user, db_password, db_url);

    let (client, connection) = tokio_postgres::connect(db_url.as_str(), NoTls)
        .await
        .unwrap();
    let arc_mutex_client = Arc::new(Mutex::new(client));

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    HttpServer::new(move || {
        let arc_mutex_client = arc_mutex_client.clone();

        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(AppState {
                client: arc_mutex_client,
            }))
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(write_to_db)
            .service(health)
            .service(get_cities)
    })
    .bind((api_addr, api_port.parse().unwrap()))?
    .run()
    .await
}
