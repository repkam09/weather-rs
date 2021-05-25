mod tray;
mod types;
mod utils;

fn main() {
    let url = "https://api.repkam09.com/api/weather/current/zip/14586";
    let weather_zip = utils::get_api::<types::WeatherZip>(url);
    assert!(weather_zip.is_ok() && !weather_zip.is_err());

    // Get the weather struct out of the result
    let weather = weather_zip.unwrap();

    // Calculate the current temp in F from Kelvin
    let kelvin = weather.main.temp;
    let fehrenheit = (kelvin * 1.8) - 459.67;
    let string_weather = format!("{:.1}°", fehrenheit);

    // Display the system tray with the weather
    tray::weather_tray(string_weather);
}
