use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct LatLong {
    pub lon: f32,
    pub lat: f32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Weather {
    pub id: i32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WeatherMain {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: i32,
    pub humidity: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WeatherZip {
    pub coord: LatLong,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: WeatherMain,
    pub name: String,
}
