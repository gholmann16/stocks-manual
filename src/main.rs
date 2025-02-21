use std::{
    // io::{prelude::*, BufReader},
    // net::{TcpListener, TcpStream},
    time::SystemTime,
    fs,
};

use rusqlite::{Connection};
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, http::header::ContentType};
use serde::{Deserialize, Serialize};

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
        .body(fs::read_to_string("website/index.html").unwrap())
}

#[get("/script.js")]
async fn script() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Content-Type", "text/javascript; charset=utf-8"))
        .body(fs::read_to_string("website/script.js").unwrap())
}

#[get("/styles.css")]
async fn styles() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Content-Type", "text/css; charset=utf-8"))
        .body(fs::read_to_string("website/styles.css").unwrap())
}

#[get("/favicon.ico")]
async fn favicon() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Content-Type", "image/x-icon"))
        .body(fs::read_to_string("website/favicon.ico").unwrap())
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

    let mut response = String::new();
    for person in person_iter {
        let json = serde_json::to_string(&person.unwrap());
        response += &json.unwrap();
        response += "\n"
    }

    HttpResponse::Ok().insert_header(("Access-Control-Allow-Origin", "*")).body(response)
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
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

// fn main() {
//     let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

//     for stream in listener.incoming() {
//         let stream = stream.unwrap();

//         handle_connection(stream);
//     }
// }

// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&stream);
//     let http_request: Vec<_> = buf_reader
//         .lines()
//         .map(|result| result.unwrap())
//         .take_while(|line| !line.is_empty())
//         .collect();

//     println!("{:#?}", http_request);

//     let status_line = "HTTP/1.1 200 OK";
//     let length = contents.len();

//     let response =
//         format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

//     stream.write_all(response.as_bytes()).unwrap();
// }