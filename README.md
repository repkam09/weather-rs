# weather-rs

A simple, hopefully cross platform/operating system, tray icon that displays the current weather.

This is a personal project for starting to learn more Rust.

# Libraries Used

`serde` for JSON => Struct mapping, see `types.rs`

`tray_item` for cross OS tray menus builder, see `tray.rs`

`reqwest` for API calls to weather service, see `utils.rs`
