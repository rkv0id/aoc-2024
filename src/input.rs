use reqwest::{
    blocking::Client,
    header::{COOKIE, USER_AGENT},
};

pub fn fetch_input(year: u16, day: u8) -> String {
    dotenvy::dotenv().ok();
    let session = std::env::var("AOC_SESSION").expect("Missing AOC_SESSION in .env or environment");
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let client = Client::new();
    let response = client
        .get(&url)
        .header(USER_AGENT, "github.com/rkv0id/aoc-2024")
        .header(COOKIE, format!("session={}", session))
        .send()
        .expect("Failed to send request");
    if !response.status().is_success() {
        panic!("Failed to fetch input: HTTP {}", response.status());
    }

    response.text().expect("Failed to read response body")
}
