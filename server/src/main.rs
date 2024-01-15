use std::{collections::HashMap, fs::File, io::BufReader};

use actix_files;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer};
use chrono::NaiveDate;
use serde::Deserialize;

const TIMESLOTS_FILE: &str = "data/timeslots.json";
const CALENDAR_FILE: &str = "data/calendar.json";

type Calendar = HashMap<NaiveDate, String>;

#[get("/api/timeslots")]
async fn api_timeslots() -> Result<actix_files::NamedFile, Error> {
    let timeslots = actix_files::NamedFile::open(TIMESLOTS_FILE)?;
    Ok(timeslots
        .use_last_modified(true)
        .set_content_type(mime::APPLICATION_JSON))
}

#[get("/api/calendar")]
async fn api_calendar_get() -> Result<actix_files::NamedFile, Error> {
    let calendar = actix_files::NamedFile::open(CALENDAR_FILE)?;
    Ok(calendar
        .use_last_modified(true)
        .set_content_type(mime::APPLICATION_JSON))
}

#[derive(Debug, Deserialize)]
struct ApiCalendarPostReq {
    date: NaiveDate,
    timeslot: Option<String>,
}

#[post("/api/calendar")]
async fn api_calendar_post(body: web::Json<ApiCalendarPostReq>) -> Result<HttpResponse, Error> {
    let calendar_file = File::open(CALENDAR_FILE)?;
    let reader = BufReader::new(calendar_file);
    let mut calendar: Calendar = serde_json::from_reader(reader)?;

    match &body.timeslot {
        Some(timeslot) => calendar.insert(body.date, timeslot.to_string()),
        None => calendar.remove(&body.date),
    };

    let calendar_file = File::create(CALENDAR_FILE)?;
    serde_json::to_writer(calendar_file, &calendar)?;

    Ok(HttpResponse::Ok().finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(api_timeslots)
            .service(api_calendar_get)
            .service(api_calendar_post)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
