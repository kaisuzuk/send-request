use crate::*;
use async_std::task;
use reqwest;
use serde_json::Value;

/// This function returns HealthCheckPath from the given WebAppsName.
fn health_check_path(health_check: Vec<HealthCheckPath>, webapps_name: &str) -> Option<String> {
    for hc in health_check {
        if hc.web_apps_name == webapps_name {
            return Some(hc.path);
        }
    }
    None
}

/// This function returns the health check URL from the given WebAppsInfo and HealthCheckPath list.
pub fn health_check_url(
    webapps_info: &WebAppsInfo,
    health_check_path_list: Vec<HealthCheckPath>,
) -> String {
    let health_check_path = health_check_path(health_check_path_list, &webapps_info.web_apps_name);
    let health_check_path = health_check_path.unwrap_or(DEFAULT_HEALTH_CHECK_PATH.to_string());

    format!("https://{}{}", webapps_info.web_apps_url, health_check_path)
}

/// This function returns the response from the given URL.
async fn getawait_request(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?.text().await?;

    Ok(response)
}

pub async fn many_requests(
    webapps_info_list: Vec<WebAppsInfo>,
    health_check_path_list: Vec<HealthCheckPath>,
) -> Vec<WebAppsInfo> {
    let mut handles = vec![];
    for web_apps_info in webapps_info_list {
        let url = health_check_url(&web_apps_info, health_check_path_list.clone());
        println!("URL: {}", url);
        handles.push(task::spawn_local(async move {
            let response = getawait_request(&url).await;
            match response {
                Ok(response) => {
                    let version = get_version_from_json(&response);
                    WebAppsInfo {
                        app_version: version,
                        ..web_apps_info
                    }
                }
                Err(_) => WebAppsInfo {
                    app_version: None,
                    ..web_apps_info
                },
            }
        }));
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }

    results
}

pub fn get_version_from_json(json: &str) -> Option<String> {
    let parsed: Value = serde_json::from_str(json).ok()?;
    parsed["version"].as_str().map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_check_path() {
        let health_check = vec![
            HealthCheckPath {
                web_apps_name: "webapp1".to_string(),
                path: "/health".to_string(),
            },
            HealthCheckPath {
                web_apps_name: "webapp2".to_string(),
                path: "/status".to_string(),
            },
        ];

        assert_eq!(
            health_check_path(health_check.clone(), "webapp1"),
            Some("/health".to_string())
        );
        assert_eq!(
            health_check_path(health_check.clone(), "webapp2"),
            Some("/status".to_string())
        );
        assert_eq!(health_check_path(health_check.clone(), "webapp3"), None);
    }

    #[test]
    fn test_health_check_url() {
        let webapps_info = WebAppsInfo {
            web_apps_name: "webapp1".to_string(),
            web_apps_url: "example.com".to_string(),
            app_name: "テスト用アプリ1".to_string(),
            app_version: None,
        };

        let health_check_path_list = vec![
            HealthCheckPath {
                web_apps_name: "webapp1".to_string(),
                path: "/health".to_string(),
            },
            HealthCheckPath {
                web_apps_name: "webapp2".to_string(),
                path: "/status".to_string(),
            },
        ];

        assert_eq!(
            health_check_url(&webapps_info, health_check_path_list.clone()),
            "https://example.com/health".to_string()
        );

        let webapps_info = WebAppsInfo {
            web_apps_name: "webapp2".to_string(),
            web_apps_url: "example.com".to_string(),
            app_name: "テスト用アプリ2".to_string(),
            app_version: None,
        };

        assert_eq!(
            health_check_url(&webapps_info, health_check_path_list.clone()),
            "https://example.com/status".to_string()
        );

        let webapps_info = WebAppsInfo {
            web_apps_name: "webapp3".to_string(),
            web_apps_url: "example.com".to_string(),
            app_name: "テスト用アプリ3".to_string(),
            app_version: None,
        };

        assert_eq!(
            health_check_url(&webapps_info, health_check_path_list.clone()),
            "https://example.com/api/health".to_string()
        );
    }
}
