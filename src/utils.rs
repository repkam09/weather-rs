use serde::de;

pub fn get_api<T: de::DeserializeOwned>(url: &str) -> Result<T, Box<dyn std::error::Error>> {
    let url_string = url.to_string();
    let resp = match reqwest::blocking::get(url_string) {
        Ok(c) => c.text()?,
        Err(e) => return Err(Box::from(e)),
    };

    let json: T = serde_json::from_str(&resp).expect("Unable to decode JSON response");
    Ok(json)
}
