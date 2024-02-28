use async_std::task;
use serde::{Deserialize, Serialize};
use std::fs;

mod libs;

pub const HEALTH_CHECK_PATH: &str = "./input/health-check-path.json";
pub const WEBAPPS_INFO_PATH: &str = "./input/webapps-info.json";
pub const DEFAULT_HEALTH_CHECK_PATH: &str = "/api/health";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebAppsInfo {
    web_apps_name: String,
    web_apps_url: String,
    app_name: String,
    app_version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HealthCheckPath {
    web_apps_name: String,
    path: String,
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

    let results = task::block_on(libs::many_requests(
        webapps.clone(),
        health_check_paths.clone(),
    ));

    println!("WebAppsName,WebAppsURL,AppName,AppVersion");
    for result in results {
        println!(
            "{},{},{},{}",
            result.web_apps_name,
            result.web_apps_url,
            result.app_name,
            result.app_version.unwrap_or("".to_string())
        );
    }
}
