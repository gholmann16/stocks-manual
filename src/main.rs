use std::{
    fs, time,
};

use rusqlite::Connection;

use actix_web::{
    get, http::header::ContentType, post, web, App, HttpResponse, HttpServer, Responder,
};

use serde::{Deserialize, Serialize};
use serde_json::json;

use charming::{
    component::{Legend, Axis},
    element::{ItemStyle, AxisType},
    series::Candlestick,
    Chart, HtmlRenderer, df,
};

type Symbol = String;
type CTime = u64;

#[derive(Debug)]
struct Person {
    id: i32,
    symbol: Symbol,
    name: String,
}

#[derive(Debug)]
struct Transaction {
    id: i32,
    buyer: Symbol,
    seller: Symbol,
    creator: Symbol,
    symbol: Symbol,
    cents: u32,
    quantity: u32,
    presented: CTime, // for simplicity of conversion between 3 different languages, using simplest type
    sold: CTime,
}

#[derive(Serialize, Deserialize, Debug)]
struct Price {
    symbol: Symbol,
    name: String,
    price: u32,
}

struct Account {
    positions: Vec<Person>,
    money: u32,
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

fn get_time() -> u64 {
    let time: u64;

    match time::SystemTime::now().duration_since(time::UNIX_EPOCH) {
        Ok(n) => time = n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }

    return time;
}

#[get("/stocks")]
async fn stock() -> impl Responder {
    let conn = Connection::open("./data.db").unwrap();
    let mut stmt = conn.prepare("SELECT * FROM person").unwrap();

    let person_iter = stmt
        .query_map([], |row| {
            Ok(Person {
                id: row.get(0)?,
                symbol: row.get(1)?,
                name: row.get(2)?,
            })
        })
        .unwrap();

    let mut prices: Vec<Price> = Vec::new();
    for person in person_iter {
        let new_person = person.unwrap();
        let mut hist = conn.prepare(&format!("SELECT * FROM transactions WHERE symbol = '{}'", new_person.symbol)).unwrap();

        let transaction_iter = hist.query_map([], |row| {
            Ok(Transaction {
                id: row.get(0)?,
                buyer: row.get(1)?,
                seller: row.get(2)?,
                symbol: row.get(3)?,
                creator: row.get(4)?,
                cents: row.get(5)?,
                quantity: row.get(6)?,
                presented: row.get(7)?,
                sold: row.get(8)?,
            })
        }).unwrap();

        /*
        * Stock price estimate algorithm:
        * completely custom, could not find anything online
        * for this first iteration it will be:
        * sigma of all stocks (price) * (quantity) * (0.98)^(days since sold) over total quantity * total weighted time                 
        */

        let current_time = get_time();
        let mut sigma = 100.0;
        let mut weight = 1.0;

        for transaction in transaction_iter {
            let clean = transaction.unwrap();
            sigma += clean.cents as f64 * clean.quantity as f64 * f64::powf(0.98, ((current_time - clean.sold) / 86400) as u32 as f64);
            weight += clean.quantity as f64 * f64::powf(0.98, ((current_time - clean.sold) / 86400) as u32 as f64);
        }

        let calculated = (sigma / weight) as u32;

        let summary = Price {
            symbol: new_person.symbol,
            name: new_person.name,
            price: calculated,
        };

        prices.push(summary);
    }

    let megajson = json!(prices);
    let response = serde_json::to_string(&megajson);

    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .body(response.unwrap())
}

fn generate_chart() {
    let new_chart = Chart::new()
    .x_axis(Axis::new().data(vec!["2017-10-24", "2017-10-25", "2017-10-26", "2017-10-27"]))
    .y_axis(Axis::new())
    .series(Candlestick::new().data(df![
        [20, 34, 10, 38],
        [40, 35, 30, 50],
        [31, 38, 33, 44],
        [38, 15, 5, 42]
    ]));

    let mut renderer = HtmlRenderer::new("main", 600, 400);
    renderer.save(&new_chart, "chart.html").unwrap();
}

#[get("/chart.html")]
async fn chart() -> impl Responder {

    if fs::metadata("chart.html").is_ok() == false {
        generate_chart();
    }

    HttpResponse::Ok()
        .insert_header(ContentType::html())
        .body(fs::read("chart.html").unwrap())
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
            .service(chart)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
