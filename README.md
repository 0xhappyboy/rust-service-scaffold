# rust-service-scaffold

rust api service scaffold

## Startup Options

Priority: **CLI args > environment variables > defaults**

### CLI Arguments

```
--port <PORT>  |  -p    Port to listen on    default: 8080
--host <HOST>  |  -h    Bind address          default: 0.0.0.0
--help                  Show help
```

### Environment Variables

```
APP_PORT    Port to listen on    default: 8080
APP_HOST    Bind address         default: 0.0.0.0
```

### Examples

```bash
# Default
# listening on http://127.0.0.1:8080
cargo run

# Environment variables
# listening on http://0.0.0.0:8080
APP_PORT=8080 APP_HOST=0.0.0.0 cargo run

# CLI args (override env)
# listening on http://0.0.0.0:9000
APP_PORT=8080 cargo run -- --port 9000 --host 0.0.0.0

# Short flags
cargo run -- -p 9000 -h 0.0.0.0

# Help
cargo run -- --help
```

### Bind Address

```
127.0.0.1    Local machine only     Local development (default)
0.0.0.0      All network interfaces Server / Docker deployment
```

> For Docker or remote deployment, bind to `0.0.0.0`, otherwise the service is unreachable from outside the container.
