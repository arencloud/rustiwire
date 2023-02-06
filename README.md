# Wireguard based SDWAN services command line tool

> **Note:**
> At this moment was fully implemented and tested tailscale device manipulation section.

### How to set configuration
>**Info:**
> Once configuration is set, tool will never ask token.

Before using tool, user has to set configuration, otherwise command line will respond with
hint how to set configuration.
```shell
% wirecli device tag 
✔ Device ID:   32094450681839615
✔ Enter tags to set (example: linux prod): linux prod
set at least single config set, executing following command: wirecli config set
```
Setting configuration with config set command
```shell
% wirecli config set   
✔ Name:   tailscale
✔ API URI:   https://api.tailscale.com
✔ API Token:   ********
```
## Device Section
### List devices connected to taiscale SDWAN
```shell
% wirecli device list      
✔ All Devices  true
✔ Scope:   egaren.org.github
┌───────────────────┬────────────┬──────────────────────────────────────┬─────────────────────┬─────────────────────────────────────────┬───────────────────┬─────────┬────────┬────────┬──────────────────────┐
│ ID                │ Authorized │ Name                                 │ Hostname            │ Addresses                               │ User              │ OS      │ Client │ Update │ Last Activity        │
├───────────────────┼────────────┼──────────────────────────────────────┼─────────────────────┼─────────────────────────────────────────┼───────────────────┼─────────┼────────┼────────┼──────────────────────┤
│ 23269467059703533 │ true       │ desktop-qkg25rh.tail07384.ts.net     │ DESKTOP-QKG25RH     │ 100.70.41.99                            │ egevorkyan@github │ windows │ 1.36.0 │ false  │ 2023-02-06T21:42:42Z │
│                   │            │                                      │                     │ fd7a:115c:a1e0:ab12:4843:cd96:6246:2963 │                   │         │        │        │                      │
├───────────────────┼────────────┼──────────────────────────────────────┼─────────────────────┼─────────────────────────────────────────┼───────────────────┼─────────┼────────┼────────┼──────────────────────┤
│ 229292790549818   │ true       │ macbook-pro.tail07384.ts.net         │ MacBook-Pro         │ 100.96.171.123                          │ egevorkyan@github │ macOS   │ 1.36.0 │ false  │ 2023-02-06T21:45:11Z │
│                   │            │                                      │                     │ fd7a:115c:a1e0:ab12:4843:cd96:6260:ab7b │                   │         │        │        │                      │
├───────────────────┼────────────┼──────────────────────────────────────┼─────────────────────┼─────────────────────────────────────────┼───────────────────┼─────────┼────────┼────────┼──────────────────────┤
│ 32094450681839615 │ true       │ netbird.tail07384.ts.net             │ netbird             │ 100.72.41.95                            │ egevorkyan@github │ linux   │ 1.36.0 │ false  │ 2023-02-06T21:40:23Z │
│                   │            │                                      │                     │ fd7a:115c:a1e0:ab12:4843:cd96:6248:295f │                   │         │        │        │                      │
├───────────────────┼────────────┼──────────────────────────────────────┼─────────────────────┼─────────────────────────────────────────┼───────────────────┼─────────┼────────┼────────┼──────────────────────┤
│ 34039791747254153 │ true       │ dev.tail07384.ts.net                 │ dev                 │ 100.106.177.92                          │ egevorkyan@github │ linux   │ 1.36.0 │ false  │ 2023-02-06T21:40:23Z │
│                   │            │                                      │                     │ fd7a:115c:a1e0:ab12:4843:cd96:626a:b15c │                   │         │        │        │                      │
└───────────────────┴────────────┴──────────────────────────────────────┴─────────────────────┴─────────────────────────────────────────┴───────────────────┴─────────┴────────┴────────┴──────────────────────┘
```
### List single device connected to taiscale SDWAN
```shell
% wirecli device list
✔ All Devices  false
✔ ID:   32094450681839615
┌───────────────────┬──────────┬─────────────────────────────────────────┬───────┬────────────────┬───────────────────┐
│ ID                │ Hostname │ Addresses                               │ OS    │ Enabled Routes │ Advertised Routes │
├───────────────────┼──────────┼─────────────────────────────────────────┼───────┼────────────────┼───────────────────┤
│ 32094450681839615 │ netbird  │ 100.72.41.95                            │ linux │ "1.1.1.0/24"   │                   │
│                   │          │ fd7a:115c:a1e0:ab12:4843:cd96:6248:295f │       │ "3.3.3.0/24"   │                   │
└───────────────────┴──────────┴─────────────────────────────────────────┴───────┴────────────────┴───────────────────┘
```
### Delete device connected to taiscale SDWAN
```shell
% wirecli device delete  
✔ Device ID:   23269467059703533
status code: 200 OK, successfully deleted!
```
### Authorize device to use tailscale SDWAN service
```shell
% wirecli device authorize
✔ Device ID:  34039791747254153
status code: 200 OK, device successfully authorized!
```
### Set ACL tag on device connected to taiscale SDWAN
```shell
ACL with tag server will be assigned to device
% wirecli device tag      
✔ Device ID:  32094450681839615
✔ Enter tags to set (example: linux prod): server
status code: 200 OK, successfully assigned tags on device!
```
### Disable or enable key expire for device
```shell
Example below makes key not expirable
% wirecli device key 
✔ Device ID:  32094450681839615
✔ Set key expire:  true
status code: 200 OK, successfully set to true!
```
```shell
Example below makes key expirable after 6 month, device must reauth to continue to use services
% wirecli device key
✔ Device ID:  32094450681839615
✔ Set key expire:  false
status code: 200 OK, successfully set to false!
```
### Set routes on device
```shell
% wirecli device route set 
✔ Device ID:  32094450681839615
✔ Enter routes to set (example: 1.1.1.0/24 2.2.2.0/30):  122.122.122.0/24 1.1.1.0/30 4.4.4.4/32
┌───────────────────┬────────────────────┬───────────────────┐
│ Device ID         │ Enabled Routes     │ Advertised Routes │
├───────────────────┼────────────────────┼───────────────────┤
│ 32094450681839615 │ "1.1.1.0/30"       │                   │
│                   │ "4.4.4.4/32"       │                   │
│                   │ "122.122.122.0/24" │                   │
└───────────────────┴────────────────────┴───────────────────┘
```
### List routes assigned to device
```shell
% wirecli device route list
✔ Device ID:  32094450681839615
┌───────────────────┬────────────────────┬───────────────────┐
│ Device ID         │ Enabled Routes     │ Advertised Routes │
├───────────────────┼────────────────────┼───────────────────┤
│ 32094450681839615 │ "1.1.1.0/30"       │                   │
│                   │ "4.4.4.4/32"       │                   │
│                   │ "122.122.122.0/24" │                   │
└───────────────────┴────────────────────┴───────────────────┘
```

>***Information:***
> tailscale full functionality will be added soon, like DNS, ACL and other features.
> Netbird functionality will be added as well, as soon as possible.