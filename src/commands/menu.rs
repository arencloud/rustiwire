use clap::{Parser, Subcommand, Args};
use crate::commands::{device, configuration};

#[derive(Debug, Parser)]
#[command(name = "wirecli")]
#[command(about = "Management command line tool for wireguard based SDWANs")]
struct WireCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// set api configuration
    Config(Config),
    /// device management
    Device(Device),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct Config {
    #[command(subcommand)]
    command: Option<ConfigSubcommands>,
}

#[derive(Debug, Subcommand)]
enum ConfigSubcommands {
    /// get configurations
    Get,
    /// set configuration
    Set(configuration::Configuration),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct Device {
    #[command(subcommand)]
    command: Option<DeviceSubcommands>,
}

#[derive(Debug, Subcommand)]
enum DeviceSubcommands {
    /// list devices
    List(device::Devices),
    /// delete device
    Delete(device::Devices),
    /// authorize machine
    Authorize(device::Devices),
    /// tag devices
    Tag(device::Devices),
    /// device keys
    Key(device::Devices),
    /// device routes
    Route(Route)
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct Route {
    #[command(subcommand)]
    command: Option<RouteSubcommands>,
}

#[derive(Debug, Subcommand)]
enum RouteSubcommands {
    /// list routes
    List(device::Devices),
    /// set routes
    Set(device::Devices),
}

pub fn commands() {
    let cmd = WireCli::parse();
    match cmd.command {
        Commands::Config(config) => {
            let config_cmd = config.command.unwrap();
            match config_cmd {
                ConfigSubcommands::Get => println!("get"),
                ConfigSubcommands::Set(c) => c.set(),
            }
        },
        Commands::Device(device) => {
            let device_cmd = device.command.unwrap();
            match device_cmd {
                DeviceSubcommands::List(l) => l.device_list(),
                DeviceSubcommands::Delete(d) => d.device_delete(),
                DeviceSubcommands::Authorize(a) => a.set_authorized(),
                DeviceSubcommands::Tag(t) => t.set_tags(),
                DeviceSubcommands::Key(k) => k.set_expire_key(),
                DeviceSubcommands::Route(route) => {
                    let route_cmd = route.command.unwrap();
                    match route_cmd {
                        RouteSubcommands::List(rl) => rl.list_routes(),
                        RouteSubcommands::Set(rs) => rs.set_routes(),
                    }
                }
            }
        }
    }
}