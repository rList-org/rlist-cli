use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// `Config` of `CloudDriver` trait.
pub struct OnedriveConfig {
    /// The refresh token for the onedrive account.
    /// *For further information, please refer to the official documentation of Microsoft OAuth 2.0 authorization flow.*
    pub refresh_token: String,

    /// The client id for the application.
    /// You can get it from the Azure portal with the client secret.
    pub client_id: String,

    /// The client secret for the application.
    /// You can get it from the Azure portal with the client id.
    pub client_secret: String,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
/// The response json when request `AUTH_URL`.
struct AccessTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: i64,
    scope: String,
    refresh_token: String,
}

// Step 1: login and get the access token

const AUTH_URL: &str = "https://login.microsoftonline.com/common/oauth2/v2.0/token";
async fn fetch_access_token(config: &OnedriveConfig) -> Result<String, String> {
    let client = reqwest::Client::new();
    let res = client.post(AUTH_URL)
        .form(&[
            ("client_id", &config.client_id),
            ("refresh_token", &config.refresh_token),
            ("requested_token_use", &"on_behalf_of".to_owned()),
            ("client_secret", &config.client_secret),
            ("grant_type", &"refresh_token".to_owned()),
        ])
        .send().await;
    match res {
        Ok(res) => {
            if let Ok(body) = res.json::<AccessTokenResponse>().await {
                return Ok(body.access_token);
            } else {
                return Err("Failed to parse response json.".to_owned());
            }
        }
        Err(e) => Err(e.to_string())
    }
}

// Step 2: get the drive id
// drive id will be stored in State with the refresh token and access token.

const MY_DRIVE_URL: &str = "https://graph.microsoft.com/v1.0/me/drive";
#[derive(Debug, Deserialize)]
/// Response json when request `MY_DRIVE_URL`.
struct MyDrive {
    id: String,
}
async fn get_my_od_id(access_token: &str) -> Result<String, String> {
    let client = reqwest::Client::new();
    let res = client.get(MY_DRIVE_URL)
        .header("Authorization", format!("Bearer {}", access_token))
        .send().await;
    match res {
        Ok(res) => {
            if let Ok(body) = res.json::<MyDrive>().await {
                return Ok(body.id);
            } else {
                return Err("Failed to parse response json.".to_owned());
            }
        }
        Err(e) => Err(e.to_string())
    }
}

/// The `State` of `CloudDriver` trait.
struct State {
    access_token: String,
    access_token_expires_in: i64,
    refresh_token: String,
    drive_id: String,
}

// Step 3: request the file list

#[derive(Debug, Deserialize)]
/// the file or folder item in the response json.
struct ResponseItem {
    id: String,
    name: String,
    size: i64,
    #[serde(rename = "@microsoft.graph.downloadUrl")]
    file_download_url: Option<String>,
    file: Option<String>,
    folder: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    last_modified_date_time: String,
}

#[derive(Debug, Deserialize)]
/// the response json when request the graphql api.
struct ResponseList {
    value: Vec<ResponseItem>,
}
fn request_list_url(dir_id: &str, drive_id: &str) -> String {
    format!("https://graph.microsoft.com/v1.0/drives/{}/items/{}/children", drive_id, dir_id)
}
async fn request_list(drive_id: &str, dir_id: &str, token: &str) -> Result<ResponseList, String> {
    let client = reqwest::Client::new();
    let res = client.get(request_list_url(dir_id, drive_id))
        .header("Authorization", format!("Bearer {}", token))
        .send().await;
    match res {
        Ok(res) => {
            let body = match res.json::<ResponseList>().await {
                Ok(body) => body,
                Err(_) => return Err("Failed to parse response".to_owned()),
            };
            Ok(body)
        }
        Err(_) => Err("Failed to request list".to_owned())
    }
}