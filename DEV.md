# Developer Documentation

## Architecture Overview

json2db is a Rust async application built on Tokio for concurrent HTTP scraping and database operations.

### Project Structure

```
src/
├── main.rs      # Entry point and async workers
└── config.rs    # Configuration parsing and CLI args
```

## Source Code Reference

### Entry Point (`src/main.rs`)

- **Database initialization SQL** (`DB_INIT`): Creates `jsonstats` table with `jsonb` type for efficient JSON storage
- **`insert_record()`**: Establishes PostgreSQL connection, initializes schema if needed, inserts JSON payload with key
- **`stats_to_db()`**: Makes HTTP GET request and passes response to database insertion
- **`start_worker()`**: Main worker loop with configurable interval per endpoint, error handling with continuation
- **`run_workers()`**: Spawns concurrent workers for each endpoint, combines CLI and file-based configs using `futures::join_all`
- **`main()`**: Tokio async runtime entry point with StructOpt CLI parsing

### Configuration (`src/config.rs`)

- **`Config` struct**: CLI arguments using StructOpt with env var fallbacks, default interval: 60 seconds
- **`DbConfig` struct**: Database-specific configuration subset for internal use
- **Config file structures**: `ConfigYaml` (root with instances array), `ScratchItem` (individual endpoint with optional interval override)
- **`Config` implementation**:
  - `read_config_file()`: Parses YAML config
  - `get_scratch_item()`: Creates optional CLI-based endpoint
  - `get_db_config()`: Returns database config subset

## Dependencies

| Crate | Purpose |
|-------|---------|
| `tokio` | Async runtime |
| `tokio-postgres` | PostgreSQL async driver |
| `reqwest` | HTTP client |
| `serde`/`serde_json`/`serde_yaml` | Serialization |
| `structopt` | CLI argument parsing |
| `anyhow` | Error handling |
| `futures` | Async utilities |

## Key Design Decisions

1. **Concurrent Workers**: Each endpoint runs in its own async task for true parallelism
2. **Error Resilience**: Individual worker failures don't crash other workers
3. **Flexible Config**: Supports both CLI args (simple) and YAML (complex multi-endpoint)
4. **JSONB Storage**: PostgreSQL's binary JSON format for efficient queries
5. **Connection Per Request**: Simple but not optimal - connections could be pooled for production

## Building from Source

Requires Rust toolchain:

```bash
# Clone the repository
git clone https://github.com/kisel/json2db.git
cd json2db

# Build release binary (~4MB)
cargo build --release

# Binary location
target/release/json2db
```

## Running Tests

```bash
cargo test
```

## Creating a Release

```bash
# Build optimized binary
cargo build --release

# Strip symbols for smaller size (optional)
strip target/release/json2db

# Create GitHub release (requires gh CLI)
gh release create v0.1.0 target/release/json2db --title "v0.1.0" --notes "Release notes"
```
