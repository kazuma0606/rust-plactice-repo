use clap::Parser;
use reqwest::Client;
use anyhow::Result;
use anyhow::Context;
use std::env;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    city: String,
}

async fn fetch_weather(city: &str) -> Result<WeatherResponse> {
    let api_key = env::var("OPENWEATHER_API_KEY")
        .with_context(|| "OPENWEATHER_API_KEYが設定されていません")?;

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    let res = Client::new()
        .get(&url)
        .send()
        .await
        .with_context(|| format!("{}の天気情報を取得できませんでした。", city))?
        .json::<WeatherResponse>()
        .await
        .with_context(|| format!("{}の天気情報をJSONに変換できませんでした。", city))?;


    Ok(res)
}


use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    name: String,
    main: Main,
    weather: Vec<Weather>,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f64,
}

#[derive(Debug, Deserialize)]
struct Weather {
    description: String,
}


#[tokio::main]
async fn main() -> Result<()> {
    dotenv::from_path("C:\\Users\\yoshi\\rust-mastery\\section3\\section3-4\\.env")
    .with_context(|| ".envファイルを読み込めませんでした")?;

    let args = Args::parse();
    let weather = fetch_weather(&args.city).await?;

    println!("都市: {}", weather.name);
    println!("気温: {}℃", weather.main.temp);
    println!("天気: {}", weather.weather[0].description);

    Ok(())
}
