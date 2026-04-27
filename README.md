# steam-protos

Rust protobuf bindings for Steam protocol messages, auto-generated from [SteamDatabase/Protobufs](https://github.com/SteamDatabase/Protobufs).

## Features

- **Auto-generated** from official Steam protobuf definitions
- **Selective compilation** via Cargo feature flags (fast builds!)
- **Zero manual maintenance** - build script downloads and compiles latest protos
- **Git-based updates** - uses `git2` for efficient incremental updates
- **Cross-platform** - works on Windows, Linux, and macOS

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
steam-protos = { path = "../steam-protos" }
prost = "0.12"
```

### Feature Flags

By default, only the `steam` module is compiled. Use feature flags to control which proto modules are built:

| Feature | Description | Proto Count | Build Time |
|---------|-------------|-------------|------------|
| `steam` (default) | Core Steam client messages | ~108 | ~2-3 min |
| `csgo` | CS:GO game coordinator | ~41 | ~1 min |
| `webui` | Steam Deck/WebUI messages | ~118 | ~2-3 min |
| `full` | All proto modules | ~267 | ~7+ min |

#### Examples

```toml
# Default: only Steam core (fastest)
steam-protos = { path = "../steam-protos" }

# Only CS:GO protos
steam-protos = { path = "../steam-protos", default-features = false, features = ["csgo"] }

# Steam + CS:GO
steam-protos = { path = "../steam-protos", features = ["csgo"] }

# Everything (slowest build)
steam-protos = { path = "../steam-protos", features = ["full"] }
```

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

## Build Process

The build script (`build.rs`) automatically:

1. **Reads feature flags** - Determines which proto directories to compile
2. **Downloads `protoc`** - Platform-specific compiler (Windows/Linux/macOS)
3. **Clones protobufs** - Uses `git2` for efficient repository management
4. **Compiles `.proto` files** - Uses `prost-build` for Rust code generation
5. **Generates module files** - Creates `mod.rs` in `src/generated/`

### Architecture

```
build.rs
├── Feature Detection (CARGO_FEATURE_*)
├── Error Handling (anyhow)
├── Cache Validation (version-based)
├── Git Operations (git2 crate)
│   ├── Clone fresh repository
│   ├── Fetch updates (incremental)
│   └── Checkout specific commit/branch
├── Protoc Management
│   ├── Download platform-specific binary
│   └── Extract and cache
└── Proto Compilation (prost-build)
    └── Per-directory compilation with progress
```

### Why Both `protoc` and `prost-build`?

They serve different roles in a two-stage pipeline:

```
.proto files  ──►  protoc (parser)  ──►  prost-build (generator)  ──►  Rust code
```

| Component | Role | Written In |
|-----------|------|------------|
| **`protoc`** | Parses `.proto` syntax into descriptors | C++ (Google's official) |
| **`prost-build`** | Converts descriptors to Rust structs | Rust |

### Configuration

Edit constants in `build.rs`:

```rust
// Protoc version
const PROTOC_VERSION: &str = "33.1";

// Protobufs commit (use "master" for latest or specific hash)
const PROTOBUFS_COMMIT: &str = "master";
```

### Caching

Downloaded files are cached in `proto_cache/`:

```
proto_cache/
├── protoc/           # Protoc compiler (version-tagged)
│   ├── bin/protoc
│   ├── include/
│   └── .version
└── Protobufs/        # Git repository (branch/commit tracked)
```

Delete `proto_cache/` to force a complete re-download.

### Build Dependencies

The build script uses these crates (in `[build-dependencies]`):

| Crate | Purpose |
|-------|---------|
| `prost-build` | Protobuf to Rust compilation |
| `git2` | Git repository operations |
| `reqwest` | HTTP downloads (protoc) |
| `zip` | Archive extraction |
| `anyhow` | Error handling |
| `tempfile` | Temporary directory management |

## Build Time Optimization

If build times are still too long, consider these strategies:

1. **Use only what you need** - Enable only required features
2. **Pre-generate code** - Run build once and commit `src/generated/` to git, then remove `build.rs`
3. **Use a specific commit** - Set `PROTOBUFS_COMMIT` to a hash to avoid git fetch on each build

## License

MIT

