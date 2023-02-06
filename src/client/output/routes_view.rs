use serde::{Deserialize, Serialize};
use serde_json::Value;
use tabled::{Style, Table, Tabled};

#[derive(Tabled)]
struct ViewTableRoutes {
    #[tabled(rename = "Device ID")]
    id:  String,

    #[tabled(rename = "Enabled Routes")]
    enabled_routes: String,

    #[tabled(rename = "Advertised Routes")]
    advertised_routes: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Routes {
    pub advertised_routes: Vec<String>,
    pub enabled_routes: Vec<Value>,
}

pub fn view_routes(device_id: String, data: &[u8]) {
    let view_data: Routes = serde_json::from_slice(data).unwrap();
    let view_routes = ViewTableRoutes {
        id: device_id,
        enabled_routes: format!("{}", view_data.enabled_routes.iter().map(|routes| format!("{}\n", routes)).collect::<String>().trim_end_matches("\n")),
        advertised_routes: format!("{}", view_data.advertised_routes.iter().map(|routes| format!("{}\n", routes)).collect::<String>().trim_end_matches("\n")),
    };
    let temp: Vec<ViewTableRoutes> = vec![view_routes];
    let mut table = Table::new(temp);
    table.with(Style::modern());
    println!("{}", table);
}

