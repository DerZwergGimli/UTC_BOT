# UTC BOT DISCORD

![bot icon](icon.drawio.png)

A simple Discord BOT displaying the current time in UTC written in Rust.

Easy deployment with Docker and ENVs.

## View

![bot view](bot_view_discord.png)

## Commands:

- `+time` Prints out the time

## Requirements:

- Docker
- Docker-Compose

## Setup:

1. Get a Discord Bot Token
    1. OAuth2
        1. SCOPES: Bot
        2. BOT PERMISSIONS: Change Nickname, Manage Nicknames

2. Clone repo
3. Move into dir `cd UTC_BOT`
4. Copy `docker.compose.yaml.sample` to `docker.compose.yaml`
5. Edit `nano docker.compose.yaml` enter your Discord-Token
6. Start with  `docker-compose up -d`
7. Invite the bot into you discord servers

## Environment Variables:

```
- DISCORD_TOKEN: <DISCORD_TOKEN>
- LOG_LEVEL: "INFO"
```

## Development:

- RustC + Cargo
- cargo install cargo-chef
- cargo chef prepare --recipe-path recipe.json

### Donations:

Solana-Wallet domain: `coffeeplease.sol`