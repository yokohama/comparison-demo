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

#[derive(Debug, Deserialize)]
struct Wind {
    speed: f32
}

#[derive(Debug, Deserialize)]
struct Main {
    temp_max: f32,
    temp_min: f32
}

#[derive(Debug, Deserialize)]
struct JsonResponse {
    cod: u8,
    name: String,
    main: Main,
    wind: Wind
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let point = env::var("POINT").expect("CITY was not found.");
    let appid = env::var("APPID").expect("APPID was not found.");
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={point}&units=metric&appid={appid}");

    let res = reqwest::get(&url).await?;
    //let body = res.text().await?;
    let body = res.json::<JsonResponse>().await?;

    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("statusCode: {}, 都市: {}, 最高気温: {}, 最低気温: {}, 風速: {}", 
          body.cod,
          body.name,
          body.main.temp_max,
          body.main.temp_min,
          body.wind.speed
        ),
    };

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
