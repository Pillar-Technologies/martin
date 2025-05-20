## Running with Docker

You can use official Docker image [`ghcr.io/maplibre/martin`](https://ghcr.io/maplibre/martin)

### Using Non-Local PostgreSQL

```bash
docker run \
  -p 3000:3000 \
  -e DATABASE_URL=postgresql://postgres@postgres.example.org/db \
  ghcr.io/maplibre/martin
```

### Exposing Local Files

You can expose local files to the Docker container using the `-v` flag.

```bash
docker run \
  -p 3000:3000 \
  -v /path/to/local/files:/files \
  ghcr.io/maplibre/martin /files
```

### Accessing Local PostgreSQL on Linux

If you are running PostgreSQL instance on `localhost`, you have to change network settings to allow the Docker container
to access the `localhost` network.

For Linux, add the `--net=host` flag to access the `localhost` PostgreSQL service. You would not need to export ports
with `-p` because the container is already using the host network.

```bash
docker run \
  --net=host \
  -e DATABASE_URL=postgresql://postgres@localhost/db \
  ghcr.io/maplibre/martin
```

### Accessing Local PostgreSQL on macOS

For macOS, use `host.docker.internal` as hostname to access the `localhost` PostgreSQL service.

```bash
docker run \
  -p 3000:3000 \
  -e DATABASE_URL=postgresql://postgres@host.docker.internal/db \
  ghcr.io/maplibre/martin
```

### Accessing Local PostgreSQL on Windows

For Windows, use `docker.for.win.localhost` as hostname to access the `localhost` PostgreSQL service.

```bash
docker run \
  -p 3000:3000 \
  -e DATABASE_URL=postgresql://postgres@docker.for.win.localhost/db \
  ghcr.io/maplibre/martin
```

### Build from source

If you need to use your own binaries inside the Docker image, compile them first
(using [`cross`](https://github.com/cross-rs/cross) or `cargo build --release`).
Then build the image using the multi-platform Dockerfile:

```bash
docker build -f .github/files/multi-platform.Dockerfile -t myuser/martin:latest .
```

Tag and push the resulting image (for example to Docker Hub or GHCR) so it can
be used in place of `ghcr.io/maplibre/martin`:

```bash
docker push myuser/martin:latest
# or
docker tag myuser/martin:latest ghcr.io/myuser/martin:latest
docker push ghcr.io/myuser/martin:latest
```
