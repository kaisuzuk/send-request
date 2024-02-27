use crate::*;
use async_std::task;
use reqwest::{self, Response};

/// This function returns HealthCheckPath from the given WebAppsName.
fn health_check_path(health_check: Vec<HealthCheckPath>, webapps_name: &str) -> Option<String> {
    for hc in health_check {
        if hc.WebAppsName == webapps_name {
            return Some(hc.Path);
        }
    }
    None
}

/// This function returns the health check URL from the given WebAppsInfo and HealthCheckPath list.
pub fn health_check_url(
    webapps_info: WebAppsInfo,
    health_check_path_list: Vec<HealthCheckPath>,
) -> String {
    let health_check_path = health_check_path(health_check_path_list, &webapps_info.WebAppsName);
    let health_check_path = health_check_path.unwrap_or(DEFAULT_HEALTH_CHECK_PATH.to_string());

    format!("https://{}{}", webapps_info.WebAppsURL, health_check_path)
}

async fn getawait_request(url: &str) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;

    Ok(response)
}

pub async fn many_requests(
    requests: Vec<String>,
) -> Vec<Result<reqwest::Response, reqwest::Error>> {
    let mut handles = vec![];
    for url in requests {
        handles.push(task::spawn_local(
            async move { getawait_request(&url).await },
        ));
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_check_path() {
        let health_check = vec![
            HealthCheckPath {
                WebAppsName: "webapp1".to_string(),
                Path: "/health".to_string(),
            },
            HealthCheckPath {
                WebAppsName: "webapp2".to_string(),
                Path: "/status".to_string(),
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
            WebAppsName: "webapp1".to_string(),
            WebAppsURL: "example.com".to_string(),
            AppName: "テスト用アプリ1".to_string(),
        };

        let health_check_path_list = vec![
            HealthCheckPath {
                WebAppsName: "webapp1".to_string(),
                Path: "/health".to_string(),
            },
            HealthCheckPath {
                WebAppsName: "webapp2".to_string(),
                Path: "/status".to_string(),
            },
        ];

        assert_eq!(
            health_check_url(webapps_info.clone(), health_check_path_list.clone()),
            "https://example.com/health".to_string()
        );

        let webapps_info = WebAppsInfo {
            WebAppsName: "webapp2".to_string(),
            WebAppsURL: "example.com".to_string(),
            AppName: "テスト用アプリ2".to_string(),
        };

        assert_eq!(
            health_check_url(webapps_info.clone(), health_check_path_list.clone()),
            "https://example.com/status".to_string()
        );

        let webapps_info = WebAppsInfo {
            WebAppsName: "webapp3".to_string(),
            WebAppsURL: "example.com".to_string(),
            AppName: "テスト用アプリ3".to_string(),
        };

        assert_eq!(
            health_check_url(webapps_info.clone(), health_check_path_list.clone()),
            "https://example.com/api/health".to_string()
        );
    }
}
