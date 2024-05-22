use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use std::env;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let point = env::var("POINT").expect("CITY was not found.");
    let appid = env::var("APPID").expect("APPID was not found.");
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={point}&units=metric&appid={appid}");

    let res = reqwest::get(&url).await?;
    let body = res.text().await?;

    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("Command {}.", body),
    };

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
