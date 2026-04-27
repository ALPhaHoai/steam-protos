# steam-protos

[![Crates.io](https://img.shields.io/crates/v/steam-protos.svg)](https://crates.io/crates/steam-protos)
[![Docs.rs](https://docs.rs/steam-protos/badge.svg)](https://docs.rs/steam-protos)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Pre-generated Rust protobuf bindings for Steam protocol messages, derived from [SteamDatabase/Protobufs](https://github.com/SteamDatabase/Protobufs).

## Features

- **Pre-generated** — no build script, no `protoc` dependency, instant compilation
- **Selective compilation** via Cargo feature flags
- **Serde support** via optional feature (enabled by default)

## Installation

```toml
[dependencies]
steam-protos = "0.1"
prost = "0.12"
```

### Feature Flags

| Feature | Description |
|---------|-------------|
| `serde` (default) | Enables `serde::Serialize`/`Deserialize` on all message types |

## Usage

### Basic Usage

```rust
use steam_protos::steam::steammessages_clientserver_login::CMsgClientLogon;
use prost::Message;

// Create a message
let logon = CMsgClientLogon {
    protocol_version: Some(65580),
    ..Default::default()
};

// Encode to bytes
let bytes = logon.encode_to_vec();

// Decode from bytes
let decoded = CMsgClientLogon::decode(bytes.as_slice()).unwrap();
```

## Module Structure

```
steam_protos
├── steam/       # Core Steam client messages (108 modules) - feature: "steam"
├── csgo/        # CS:GO game coordinator messages (41 modules) - feature: "csgo"
└── webui/       # Steam Deck/WebUI messages (118 modules) - feature: "webui"
```

> **Note:** Modules are only available if their corresponding feature is enabled.

### Key Steam Modules

| Module | Purpose |
|--------|---------|
| `steammessages_clientserver_login` | Login, logoff, heartbeat |
| `steammessages_clientserver` | Games played, licenses, wallet |
| `steammessages_clientserver_friends` | Friends list, persona state |
| `steammessages_clientserver_2` | Rich presence, trading, notifications |
| `steammessages_auth_steamclient` | Authentication service |
| `steammessages_twofactor_steamclient` | Two-factor authentication |
| `steammessages_chat_steamclient` | Chat messages |
| `steammessages_cloud_steamclient` | Cloud storage |

### Example Imports

```rust
// Login messages
use steam_protos::steam::steammessages_clientserver_login::{
    CMsgClientLogon, CMsgClientLogonResponse,
};

// Authentication
use steam_protos::steam::steammessages_auth_steamclient::{
    CAuthenticationBeginAuthSessionViaCredentialsRequest,
    CAuthenticationGetPasswordRSAPublicKeyRequest,
};

// Friends
use steam_protos::steam::steammessages_clientserver_friends::{
    CMsgClientFriendsList, CMsgClientPersonaState,
};

// Two-Factor Auth
use steam_protos::steam::steammessages_twofactor_steamclient::{
    CTwoFactorAddAuthenticatorRequest, CTwoFactorAddAuthenticatorResponse,
};
```

## License

MIT

