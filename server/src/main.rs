use actix_files as fs;
use actix_web::{get, App, Error, HttpServer};

#[get("/api/timeslots")]
async fn api_timeslots() -> Result<fs::NamedFile, Error> {
    let timeslots = fs::NamedFile::open("data/timeslots.json")?;
    Ok(timeslots
        .use_last_modified(true)
        .set_content_type(mime::APPLICATION_JSON))
}

#[get("/api/calendar")]
async fn api_calendar() -> Result<fs::NamedFile, Error> {
    let calendar = fs::NamedFile::open("data/calendar.json")?;
    Ok(calendar
        .use_last_modified(true)
        .set_content_type(mime::APPLICATION_JSON))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(api_timeslots).service(api_calendar))
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}
