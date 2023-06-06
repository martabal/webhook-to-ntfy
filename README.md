# rust-ntfy

[![Publish Release](https://github.com/martabal/webhook-to-ntfy/actions/workflows/push_docker.yml/badge.svg)](https://github.com/martabal/webhook-to-ntfy/actions/workflows/push_docker.yml)
[![Build](https://github.com/martabal/webhook-to-ntfy/actions/workflows/build.yml/badge.svg)](https://github.com/martabal/webhook-to-ntfy/actions/workflows/build.yml)

## What is it

Use [ntfy](https://github.com/binwiederhier/ntfy) to services that don't support ntfy, but support webhook notifications.

Services available :

- gitea
- grafana
- overseerr

## Run it

### Docker-cli ([click here for more info](https://docs.docker.com/engine/reference/commandline/cli/))

```sh
docker run --name=webhook-to-ntfy \
    -e NTFY_BASE_URL=http://192.168.1.10:8080 \
    -e NTFY_USERNAME=admin \
    -e NTFY_PASSWORD='<your_password>' \
    -e PORT=3000 `#optional` \
    -p 3000:3000 \
    martabal/webhook-to-ntfy
```

### Docker-compose

```yaml
version: "2.1"
services:
  immich:
    image: martabal/webhook-to-ntfy:latest
    container_name: webhook-to-ntfy
    environment:
      - NTFY_BASE_URL=http://192.168.1.10:8080
      - NTFY_USERNAME=admin
      - NTFY_PASSWORD='<your_password>'
      - PORT=3000 #optional
    volumes:
      - ~/webhook-to-ntfy:/config
    ports:
      - 3000:3000
    restart: unless-stopped
```

### Without docker

```sh
git clone https://github.com/martabal/webhook-to-ntfy.git
cd webhook-to-ntfy
cargo build --release
./target/release/webhookntfy
```
