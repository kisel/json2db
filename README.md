# json2db

A lightweight Rust application (~4MB) that fetches JSON data from HTTP endpoints and stores it in PostgreSQL.

## What it does

json2db periodically scrapes JSON data from configured API endpoints and persists it to a PostgreSQL database. Each record is stored with a key identifier and timestamp, making it useful for monitoring, metrics collection, or data archival.

## Features

- **Lightweight**: Small binary (~4MB), low memory footprint
- **Concurrent**: Fetch from multiple HTTP endpoints in parallel
- **Flexible**: CLI arguments for simple use, YAML config for complex setups
- **Efficient**: Uses PostgreSQL JSONB for fast queries
- **Docker ready**: Single container deployment

## Installation

### Download Pre-built Binary

Get the latest release from [GitHub Releases](https://github.com/kisel/json2db/releases).

### Using Docker

```bash
docker run -e DATABASE_URL="postgres://user:pass@host:5432/db" \
           -e API_URL="http://api/stats" \
           -e KEY="my-stats" \
           ghcr.io/kisel/json2db:latest
```

Or build locally:

```bash
docker build -t json2db .
```

## Configuration

### Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | `postgres://user:pass@localhost:5432/dbname` |
| `API_URL` | Endpoint to scrape (CLI mode) | `http://api.example.com/stats` |
| `KEY` | Identifier for stored records | `production-stats` |
| `INTERVAL` | Scrape interval in seconds | `60` |
| `CONFIG` | Path to YAML config file | `/config/scrape.yaml` |

### YAML Configuration

For multiple endpoints, use a config file:

```yaml
# scrape.yaml
interval: 60  # default interval

instances:
  - key: api-stats
    url: http://api1.example.com/stats
    interval: 30  # override default
  
  - key: service-metrics
    url: http://api2.example.com/metrics
    # uses default interval
```

## Usage Examples

### Single Endpoint (CLI)

```bash
export DATABASE_URL="postgres://user:pass@localhost:5432/mydb"
export API_URL="http://localhost:8080/api/v1/stats"
export KEY="my-app-stats"
export INTERVAL=60

./json2db
```

### Multiple Endpoints (Config File)

```bash
export DATABASE_URL="postgres://user:pass@localhost:5432/mydb"
export KEY="default-key"

./json2db --config /path/to/scrape.yaml
```

## Database Schema

The application creates the following table automatically:

```sql
CREATE TABLE IF NOT EXISTS jsonstats (
    id serial not null,
    key text not null,
    data jsonb,
    timestamp timestamp default current_timestamp not null
);
```

## Querying Data

Example queries:

```sql
-- Get latest record for a key
SELECT * FROM jsonstats 
WHERE key = 'my-app-stats' 
ORDER BY timestamp DESC 
LIMIT 1;

-- Get records from last hour
SELECT * FROM jsonstats 
WHERE key = 'my-app-stats' 
  AND timestamp > NOW() - INTERVAL '1 hour';

-- Extract specific JSON field
SELECT 
    key,
    data->>'field_name' as field_value,
    timestamp
FROM jsonstats 
WHERE key = 'my-app-stats';
```

## Developer Info

See [DEV.md](DEV.md) for architecture details and build instructions.

## License

MIT
