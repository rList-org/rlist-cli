pub mod loader;

use rlist_drivers::driver_index::DriverIndex;
use serde::{Deserialize, Serialize};

pub const CONFIG_FILE_PATH: &str = "config.json";


#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_address")]
    pub address: String,

    #[serde(default = "default_port")]
    pub port: u16,

    pub site_profile: SiteProfile,

    pub drives: Vec<DriverIndex>,

}

fn default_address() -> String {
    "0.0.0.0".to_string()
}
fn default_port() -> u16 {
    11451
}

fn default_site_description() -> String {
    "A new rList share site".to_string()
}
#[derive(Deserialize, Serialize, Clone)]
pub struct SiteProfile {
    pub site_name: String,

    #[serde(default = "default_site_description")]
    pub site_description: String,

    #[serde(default)]
    pub site_keywords: Vec<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic_des() {
        let config = r#"
        {
            "site_profile": {
                "site_name": "test"
            },
            "drives": []
        }
        "#;
        let parsed = serde_json::from_str::<Config>(config);
        assert!(parsed.is_ok());
    }
}