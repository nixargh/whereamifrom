# WhereAmI

## Why

An elementary tool to detect Linux active interfaces change and get your a country your public IP belongs to.
I need this as I often use some VPN tunnel and want to see my current "location" at i3 status bar.

## How it works

The code:
- Checks number of active network connections.
- If number changed app does HTTP request to external API returning geo location of IP address from where request comes.
- Updates location country at `/tmp/whereami` file.

Currently [https://ipinfo.io/](https://ipinfo.io/products/ip-geolocation-api) is used.
