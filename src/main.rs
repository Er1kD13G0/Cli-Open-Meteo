use clap::Parser;
use reqwest;
use serde::Deserialize;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'y', long)]
    latitude: f64,

    #[arg(short = 'x', long)]
    longitude: f64,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    current_weather: CurrentWeather,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    temperature: f64,
    windspeed: f64,
    winddirection: f64,
    weathercode: u8,
    time: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Monte a URL com os parâmetros de latitude e longitude e o parâmetro current_weather.
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
        args.latitude, args.longitude
    );

    println!(
        "Consultando o clima para latitude {} e longitude {}...",
        args.latitude, args.longitude
    );

    // Realize a requisição HTTP GET.
    let response = reqwest::get(&url).await?;

    if !response.status().is_success() {
        eprintln!("Falha ao obter dados: {}", response.status());
        return Ok(());
    }

    // Desserializa a resposta JSON.
    let api_response: ApiResponse = response.json().await?;

    // Exiba os dados obtidos.
    println!("Clima atual:");
    println!("Tempo: {}", api_response.current_weather.time);
    println!(
        "Temperatura: {} °C",
        api_response.current_weather.temperature
    );
    println!(
        "Velocidade do vento: {} km/h",
        api_response.current_weather.windspeed
    );
    println!(
        "Direção do vento: {}°",
        api_response.current_weather.winddirection
    );
    println!(
        "Código do clima: {}",
        api_response.current_weather.weathercode
    );

    Ok(())
}
