use clap::Parser;
use dialoguer::theme::ColorfulTheme;
use crate::client::devices;
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
pub struct Devices;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Routes {
    pub routes: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Authorized {
    pub authorized: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyExpire {
    pub key_expiry_disabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tags {
    pub tags: Vec<String>,
}

impl Devices {
    pub fn device_list(&self) {
        let theme = ColorfulTheme::default();
        let mut input_bool = dialoguer::Input::<bool>::with_theme(&theme);
        let all = input_bool.with_prompt("All Devices: ").default(false).interact().unwrap();
        match all {
            true => {
                let mut input = dialoguer::Input::<String>::with_theme(&theme);
                let scope = input.with_prompt("Scope: ").allow_empty(false).interact_text().unwrap();
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async {
                        devices::list_devices(scope).await;
                    });

            }
            false => {
                let mut input = dialoguer::Input::<String>::with_theme(&theme);
                let id = input.with_prompt("ID: ").allow_empty(false).interact_text().unwrap();
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async {
                        devices::list_device(id).await;
                    });
            }
        }
    }
    pub fn device_delete(&self) {
        let theme = ColorfulTheme::default();
        let mut input = dialoguer::Input::<String>::with_theme(&theme);
        let id = input.with_prompt("Device ID: ").allow_empty(false).interact_text().unwrap();
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                devices::delete_device(id).await;
            });
    }
    pub fn list_routes(&self) {
        let theme = ColorfulTheme::default();
        let mut input = dialoguer::Input::<String>::with_theme(&theme);
        let id = input.with_prompt("Device ID: ").allow_empty(false).interact_text().unwrap();
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                devices::list_routes(id).await;
            });
    }
    pub fn set_routes(&self) {
        let theme = ColorfulTheme::default();
        let mut input = dialoguer::Input::<String>::with_theme(&theme);
        let id = input.with_prompt("Device ID: ").allow_empty(false).interact_text().unwrap();
        let routes = input.with_prompt("Enter routes to set (example: 1.1.1.0/24 2.2.2.0/30): ")
            .allow_empty(false).interact_text().unwrap().split(" ").map(|s| s.to_string()).collect::<Vec<_>>();
        let r = Routes {
            routes,
        };
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                devices::set_routes(id, serde_json::to_string(&r).unwrap()).await;
            });
    }
    pub fn set_authorized(&self) {
        let theme = ColorfulTheme::default();
        let mut input = dialoguer::Input::<String>::with_theme(&theme);
        let id = input.with_prompt("Device ID: ").allow_empty(false).interact_text().unwrap();
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let data = Authorized {
                    authorized: true,
                };
                devices::set_authorized(id, serde_json::to_string(&data).unwrap()).await;
            });
    }
    pub fn set_tags(&self) {
        let theme = ColorfulTheme::default();
        let mut input = dialoguer::Input::<String>::with_theme(&theme);
        let id = input.with_prompt("Device ID: ").allow_empty(false).interact_text().unwrap();
        let tags = input.with_prompt("Enter tags to set (example: linux prod): ")
            .allow_empty(false).interact_text().unwrap().split(" ").map(|s| format!("tag:{}", s)).collect::<Vec<_>>();
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let tag = Tags {
                    tags,
                };
                devices::set_tags(id, serde_json::to_string(&tag).unwrap()).await;
            });
    }
    pub fn set_expire_key(&self) {
        let theme = ColorfulTheme::default();
        let mut input = dialoguer::Input::<String>::with_theme(&theme);
        let id = input.with_prompt("Device ID: ").allow_empty(false).interact_text().unwrap();
        let mut input_bool = dialoguer::Input::<bool>::with_theme(&theme);
        let key_expiry_disabled = input_bool.with_prompt("Set key expire: ").interact().unwrap();
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let exp = KeyExpire {
                    key_expiry_disabled: key_expiry_disabled,
                };
                devices::set_key_expire(id, serde_json::to_string(&exp).unwrap(), key_expiry_disabled).await;
            });
    }
}