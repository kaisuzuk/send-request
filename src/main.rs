use async_std::task;
use libs::many_requests;
use serde::{Deserialize, Serialize};
use std::fs;

mod libs;

pub const HEALTH_CHECK_PATH: &str = "./input/health-check-path.json";
pub const WEBAPPS_INFO_PATH: &str = "./input/webapps-info.json";
pub const DEFAULT_HEALTH_CHECK_PATH: &str = "/api/health";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebAppsInfo {
    WebAppsName: String,
    WebAppsURL: String,
    AppName: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HealthCheckPath {
    WebAppsName: String,
    Path: String,
}

fn main() {
    let fs =
        fs::read_to_string(HEALTH_CHECK_PATH).expect("Failed to read the health-check json file.");

    let health_check_paths: Vec<HealthCheckPath> =
        serde_json::from_str(&fs).expect("Failed to parse the health-check json file.");

    let fs =
        fs::read_to_string(WEBAPPS_INFO_PATH).expect("Failed to read the webapps-info json file.");

    let webapps: Vec<WebAppsInfo> =
        serde_json::from_str(&fs).expect("Failed to parse the webapps-info json file.");

    let mut urls = vec![];

    for webapp in webapps {
        urls.push(libs::health_check_url(webapp, health_check_paths.clone()));
    }

    let results = task::block_on(many_requests(urls));

    for result in &results {
        match result {
            Ok(response) => {
                println!("{}", response.status());
            }
            Err(err) => {
                println!("NG");
                eprintln!("Err {}", err);
            }
        }
    }
}
