# Where Am I From

## Why

An elementary tool to detect Linux active interfaces change and get a country yours public IP belongs to.  
I need this as I often use some VPN tunnel and want to see my current "location" at i3 status bar.

## How it works

The code:
- Checks number of active network connections.
- If number changed app does HTTP request to external API returning geo location of IP address from where request comes.
- Updates location country at `/tmp/whereamifrom` file.

Currently [https://ipinfo.io/](https://ipinfo.io/products/ip-geolocation-api) is used.

## Usage

Using command line arguments or environment variable you may change application behavior. For example to get more information and use it any way you like:
```shell
$ FILE=/tmp/test URL=http://ipinfo.io/ ./target/debug/whereamifrom
```
Creates:
```json
{
  "ip": "77.88.99.100",
  "city": "Orgrimmar",
  "region": "Durotar",
  "country": "Kalimdor",
  "loc": "45.00,65.00",
  "org": "The Horde",
  "postal": "0000",
  "timezone": "Azeroth",
  "readme": "https://ipinfo.io/missingauth"
}
```
