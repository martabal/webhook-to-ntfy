# rust-ntfy

[![Publish Release](https://github.com/martabal/webhook-to-ntfy/actions/workflows/push_docker.yml/badge.svg)](https://github.com/martabal/webhook-to-ntfy/actions/workflows/push_docker.yml)
[![Build](https://github.com/martabal/webhook-to-ntfy/actions/workflows/build.yml/badge.svg)](https://github.com/martabal/webhook-to-ntfy/actions/workflows/build.yml)

## How to use it

Edit config.yaml to use the services you want. Then rename it config.yaml. 

Services available :

- gitea
- grafana
- overseerr

## Run it

### Docker-cli ([click here for more info](https://docs.docker.com/engine/reference/commandline/cli/))

```sh
docker run --name=webhook-to-ntfy \
    -e QBITTORRENT_URL=http://192.168.1.10:8080 \
    -e QBITTORRENT_PASSWORD='<your_password>' \
    -e QBITTORRENT_USERNAME=admin \
    -p 8090:8090 \
    martabal/webhook-to-ntfy
```

### Docker-compose

```yaml
version: "2.1"
services:
  immich:
    image: martabal/qbittorrent-exporter:latest
    container_name: qbittorrent-exporter
    environment:
        - ~/webhook-to-ntfy/config.yaml:/config/config.yaml
    ports:
      - 8090:8090
    restart: unless-stopped
```

### Without docker

```sh
git clone https://github.com/martabal/webhook-to-ntfy.git
cd webhook-to-ntfy
cargo build --release
./target/release/webhookntfy
```
