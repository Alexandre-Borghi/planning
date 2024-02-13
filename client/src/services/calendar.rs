use chrono::NaiveDate;
use gloo::net::http::Request;

pub async fn update_day(date: NaiveDate, timeslot: Option<&str>) {
    let body = format!(
        "{{\"date\": \"{date}\", \"timeslot\": {}}}",
        timeslot
            .map(|s| format!("\"{s}\""))
            .unwrap_or("null".to_string())
    );

    Request::post("/api/calendar")
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap()
        .send()
        .await
        .expect("post request to calendar returned an error");
}
