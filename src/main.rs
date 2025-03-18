use std::{
    // io::{prelude::*, BufReader},
    // net::{TcpListener, TcpStream},
    time::SystemTime,
    fs,
};

use rusqlite::{Connection};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, http::header::ContentType};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug)]
enum OrderType {
    Buy,
    Sell,
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    id: i32,
    symbol: String,
    name: String,
}

#[derive(Debug)]
struct Transaction {
    id: i32,
    buyer: Person,
    seller: Person,
    stock: Person,
    cents: i32,
    presented: SystemTime,
    sold: SystemTime,
    kind: OrderType,
}

struct Order {
    id: i32,
    creator: Person,
    stock: Person,
    date: SystemTime,
    cents: i32,
    kind: OrderType,
}

struct Account {
    id: i32,
    positions: Vec<Person>,
    money: i32,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(ContentType::html())
        .body(fs::read("website/index.html").unwrap())
}

#[get("/script.js")]
async fn script() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Content-Type", "text/javascript; charset=utf-8"))
        .body(fs::read("website/script.js").unwrap())
}

#[get("/styles.css")]
async fn styles() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Content-Type", "text/css; charset=utf-8"))
        .body(fs::read("website/styles.css").unwrap())
}

#[get("/favicon.ico")]
async fn favicon() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Content-Type", "image/x-icon"))
        .body(fs::read("website/favicon.ico").unwrap())
}

#[get("/stocks")]
async fn stock() -> impl Responder {
    let conn = Connection::open("./data.db").unwrap();
    let mut stmt = conn.prepare("SELECT id, symbol, name FROM person").unwrap();

    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            symbol: row.get(1)?,
            name: row.get(2)?,
        })
    }).unwrap();

    let mut people: Vec<Person> = Vec::new();
    for person in person_iter {
        people.push(person.unwrap());
    }

    let megajson = json!(people);
    let response = serde_json::to_string(&megajson);

    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .body(response.unwrap())
}

#[get("/price/{id}")]
async fn get_price(path: web::Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();

    // let conn = Connection::open("./data.db").unwrap();
    // let mut stmt = conn.prepare("SELECT stock, cents FROM transactions WHERE Person").unwrap();
    HttpResponse::Ok()
        .insert_header(ContentType::plaintext())
        .body("999")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(stock)
            .service(script)
            .service(styles)
            .service(favicon)
            .service(get_price)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
