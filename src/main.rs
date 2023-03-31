use actix_web::{get, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio_postgres::{Client, NoTls};

#[derive(Deserialize)]
struct Config {
    pghost: String,
    pgpassword: String,
    pguser: String,
    pgdbname: String,
}

#[derive(Deserialize, Serialize)]
struct TableSchema {
    table_name: String,
    column_names: Vec<String>,
}

#[derive(Deserialize)]
struct Schema {
    sheets: std::collections::HashMap<String, TableSchema>,
}

async fn get_all_tables(client: &Client, schema: &Schema) -> anyhow::Result<serde_json::Value> {
    let mut result = json!({});
    for (table_name, table_schema) in schema.sheets.iter() {
        let column_names = table_schema.column_names.join(", ");
        let sql_query = format!("SELECT {} FROM {}", column_names, table_name);
        let rows = client.query(sql_query.as_str(), &[]).await?;
        let mut json_rows = Vec::new();
        for row in rows {
            let mut json_row = serde_json::Map::new();
            for (i, column_name) in table_schema.column_names.iter().enumerate() {
                let column_value: Option<String> = row.get(i);
                let json_value = match column_value {
                    Some(value) => serde_json::Value::String(value),
                    None => serde_json::Value::Null,
                };
                json_row.insert(column_name.clone(), json_value);
            }
            json_rows.push(serde_json::Value::Object(json_row));
        }
        result[table_name] = serde_json::Value::Array(json_rows);
    }
    Ok(result)
}

#[get("/alltables")]
async fn all_tables() -> HttpResponse {
    let schema_str = std::fs::read_to_string("schema.json").unwrap();
    let schema: Schema = serde_json::from_str(&schema_str).unwrap();

    let config_str = std::fs::read_to_string("config.json").unwrap();
    let config: Config = serde_json::from_str(&config_str).unwrap();

    let (client, connection) = tokio_postgres::connect(
        format!(
            "host={} user={} password={} dbname={}",
            config.pghost, config.pguser, config.pgpassword, config.pgdbname
        )
        .as_str(),
        NoTls,
    )
    .await
    .unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let all_tables = get_all_tables(&client, &schema).await.unwrap();
    HttpResponse::Ok().json(all_tables)
}

#[get("/")]
async fn helloworld() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/robots.txt")]
async fn robotstxt() -> HttpResponse {
    HttpResponse::Ok().body("User-agent: *\nDisallow: /")
}

#[get("/time")]

async fn current_time() -> HttpResponse {
    // Set up a PostgreSQL connection

    let config_str = std::fs::read_to_string("config.json").unwrap();
    let config: Config = serde_json::from_str(&config_str).unwrap();


    let (client, connection) =
        tokio_postgres::connect(format!(
            "host={} user={} password={} dbname={}",
            config.pghost, config.pguser, config.pgpassword, config.pgdbname
        )
        .as_str(), NoTls)
            .await
            .unwrap();

    // Spawn a tokio task to run the connection in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Error connecting to PostgreSQL: {}", e);
        }
    });

    // Query the current time from PostgreSQL
    let row = client
        .query_one("SELECT NOW()", &[])
        .await
        .unwrap();

    // Extract the time from the row
    let time: String = row.get(0);

    // Return an HTTP response with the time
    HttpResponse::Ok().body(time)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
    .service(all_tables)
    .service(helloworld)
    .service(robotstxt)
    .service(current_time)
)
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
