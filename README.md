# Market Dashboard
A front-end for the [market API](https://github.com/minesoftcc/market-api) created by
[me](https://github.com/STBoyden). Designed to work alongside the
[bank API](https://github.com/minesoftcc/ccash) developed by
[EntireTwix](https://github.com/EntireTwix). Though designed specifically for modded
Minecraft; it is generic enough to not only work with vanilla Minecraft, with no
source code changes, but with other sandbox/surivival games (such as Rust), with little
source code changes.

### Requirements:
- Linux
    - Debian/Ubuntu-based distros: `sudo apt install libxcb-render0-dev
        libxcb-shape0-dev libxcb-xfixes0-dev`

### Features
- Purchase and Sell in-game items.
- Create users for the bank API.
- Refreshes data every 30s.

### Todo
- **Instances**: have the ability to save multiple configurations of different market API
    servers. For example, one instance for a modded Minecraft server with it's own
    bank and another for a vanilla Minecraft server with it's own bank.

### Building and Installing
`market-dashboard` requires some environment variables for it to be able to built (this
will change somewhat). For now however, the following environment variables need to
be set:

- `ADMIN_PASS`: The admin password for the  bank API (required due to the create user
    functionality).
- `BANK_API_ADDR`: The IP or URL of the bank API instance (must start with `http://` or
    `https://` and port should be specified).
- `MARKET_API_ADDR`: The IP or URL of the market API instance (must start with `http://`
    or `https://` and port should be specified).

Once all the required environment variables are set, building `market-dashboard` should
be as simple as `cargo build --release`. Or alternatively if you want to install it to
`PATH`, `cargo install --git https://github.com/minesoftcc/market-dashboard`. If the
environment variables are not set, `market-dashboard` will fail to build in release mode.
