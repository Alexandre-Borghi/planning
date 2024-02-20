use gloo::net::http::Request;

pub async fn update(name: &str, color: &str) {
    let body = format!("\"{color}\"");

    Request::post(&format!("/api/timeslots/{name}"))
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap()
        .send()
        .await
        .expect("post request to calendar returned an error");
}
