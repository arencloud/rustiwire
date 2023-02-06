use serde::{Serialize, Deserialize};
use tabled::{Style, Table, Tabled};
use serde_json::Value;

#[derive(Tabled)]
struct ViewTableDevice {
    #[tabled(rename = "ID")]
    id:  String,

    #[tabled(rename = "Hostname")]
    host: String,

    #[tabled(rename = "Addresses")]
    add: String,

    #[tabled(rename = "OS")]
    os: String,

    #[tabled(rename = "Enabled Routes")]
    enabled_routes: String,

    #[tabled(rename = "Advertised Routes")]
    advertised_routes: String,
}

#[derive(Tabled)]
struct ViewTableDeviceList {
    #[tabled(rename = "ID")]
    id:  String,

    #[tabled(rename = "Authorized")]
    authorized: bool,

    #[tabled(rename = "Name")]
    name:  String,

    #[tabled(rename = "Hostname")]
    host: String,

    #[tabled(rename = "Addresses")]
    add: String,

    #[tabled(rename = "User")]
    user: String,

    #[tabled(rename = "OS")]
    os: String,

    #[tabled(rename = "Client")]
    client_version: String,

    #[tabled(rename = "Update")]
    update_available: bool,

    #[tabled(rename = "Last Activity")]
    last_seen: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootDeviceEntity {
    pub devices: Vec<Device>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub addresses: Vec<String>,
    pub authorized: bool,
    pub blocks_incoming_connections: bool,
    pub client_version: String,
    pub created: String,
    pub expires: String,
    pub hostname: String,
    pub id: String,
    pub is_external: bool,
    pub key_expiry_disabled: bool,
    pub last_seen: String,
    pub machine_key: String,
    pub name: String,
    pub node_id: String,
    pub node_key: String,
    pub os: String,
    pub tailnet_lock_error: String,
    pub tailnet_lock_key: String,
    pub update_available: bool,
    pub user: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleDevice {
    pub addresses: Vec<String>,
    pub id: String,
    pub user: String,
    pub name: String,
    pub hostname: String,
    pub client_version: String,
    pub update_available: bool,
    pub os: String,
    pub created: String,
    pub last_seen: String,
    pub key_expiry_disabled: bool,
    pub expires: String,
    pub authorized: bool,
    pub is_external: bool,
    pub machine_key: String,
    pub node_key: String,
    pub blocks_incoming_connections: bool,
    pub enabled_routes: Vec<Value>,
    pub advertised_routes: Vec<Value>,
    pub client_connectivity: ClientConnectivity,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientConnectivity {
    pub endpoints: Vec<String>,
    pub derp: String,
    pub client_supports: ClientSupports,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientSupports {
    pub hair_pinning: bool,
    pub ipv6: bool,
    pub pcp: bool,
    pub pmp: bool,
    pub udp: bool,
    pub upnp: bool,
}

pub fn view_list(data: &[u8]) {
    let view_data: RootDeviceEntity = serde_json::from_slice(data).unwrap();
    let view_table:  Vec<_> = view_data.devices.iter().map(|view| ViewTableDeviceList {
        id: view.id.clone(),
        name: view.name.clone(),
        host: view.hostname.clone(),
        add: format!("{}", view.addresses.iter().map(|addr| format!("{}\n", addr)).collect::<String>().trim_end_matches("\n")),
        os: view.os.clone(),
        user: view.user.clone(),
        client_version: view.client_version.clone().split_once("-").unwrap().0.to_string(),
        update_available: view.update_available,
        last_seen: view.last_seen.clone(),
        authorized: view.authorized,

    }).collect();
    let mut table = Table::new(view_table);
    table.with(Style::modern());
    println!("{}", table);
}

pub fn view_device(data: &[u8]) {
    let view_data: SingleDevice = serde_json::from_slice(data).unwrap();
    let view_device = ViewTableDevice {
        id: view_data.id,
        host: view_data.hostname,
        add: format!("{}", view_data.addresses.iter().map(|addr| format!("{}\n", addr)).collect::<String>().trim_end_matches("\n")),
        os: view_data.os,
        enabled_routes: format!("{}", view_data.enabled_routes.iter().map(|routes| format!("{}\n", routes)).collect::<String>().trim_end_matches("\n")),
        advertised_routes: format!("{}", view_data.advertised_routes.iter().map(|routes| format!("{}\n", routes)).collect::<String>().trim_end_matches("\n")),
    };
    let temp: Vec<ViewTableDevice> = vec![view_device];
    let mut table = Table::new(temp);
    table.with(Style::modern());
    println!("{}", table);
}