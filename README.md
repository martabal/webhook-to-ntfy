FORMAT: 1A
HOST: https://webhook.REDACTED.com/

# 📡 Webhook to ntfy

[![Publish Release](https://github.com/martabal/webhook-to-ntfy/actions/workflows/push_docker.yml/badge.svg)](https://github.com/martabal/webhook-to-ntfy/actions/workflows/push_docker.yml)
[![Build](https://github.com/martabal/webhook-to-ntfy/actions/workflows/build.yml/badge.svg)](https://github.com/martabal/webhook-to-ntfy/actions/workflows/build.yml)
[![Test](https://github.com/martabal/webhook-to-ntfy/actions/workflows/test.yml/badge.svg)](https://github.com/martabal/webhook-to-ntfy/actions/workflows/test.yml)

## ❔️ What is it

Use Webhook to ntfy to services that don't support [ntfy](https://github.com/binwiederhier/ntfy), but support webhook notifications.

Services available :

- [gitea](https://about.gitea.com/)
- [grafana](https://grafana.com/)
- [Overseerr](https://overseerr.dev/)

## 🏃 Run it

### 🐳 Docker-cli ([click here for more info](https://docs.docker.com/engine/reference/commandline/cli/))

```sh
docker run --name=webhook-to-ntfy \
    -e NTFY_BASE_URL=http://192.168.1.10:8080 \
    -e NTFY_USERNAME=admin \
    -e NTFY_PASSWORD='<your_password>' \
    -e PORT=3000 `#optional` \
    -p 3000:3000 \
    martabal/webhook-to-ntfy
```

### 🐋 Docker-compose

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

### 🖥 Without docker

```sh
git clone https://github.com/martabal/webhook-to-ntfy.git
cd webhook-to-ntfy
cargo build --release
./target/release/webhookntfy
```

## 🧪 Test it
Curl with username and password
```sh
curl -u username:password http://localhost:3000/healthcheck
```
Curl where username and password are require interactive
```sh
curl --header 'Authorization: Basic dXNlcm5hbWU6cGFzc3dvcmQ=' http://localhost:3000/healthcheck
```
Or open the following in your browser: [http://localhost:3000/](http://localhost:3000/) and you should see:
```
App is healthy
```

### 📡 Response
It should return a JSON object wutg the http status code as well as the same code in the header.

```json
{"status":200}
```

## 🌐 Environment Variables

To run this project, you will need to add the following environment variables to your .env file

- `NTFY_BASE_URL` - The base URL for the NTFY service.
- `NTFY_USERNAME` - Your NTFY service username.
- `NTFY_PASSWORD` - Your NTFY service password.
- `PORT` - The port number the application will listen on.

### ⚙️ Executable variables
**-e**
: This forces the environment variables to be loaded from .env file

### ⚙️ Config file variables

Below are the configurable options for each service in the `config.yaml` file:

| Service    | Auth Options | Config Options                 | Description                                 |
|------------|--------------|--------------------------------|---------------------------------------------|
| `grafana`  | username, password | icon, topic, priority, button | Configure Grafana notifications. `icon` and `priority` are optional; `button` is a boolean. |
| `gitea`    | username, password | icon, topic, priority         | Gitea service configuration. All options are optional. |
| `overseerr`| username, password | icon, topic, message, title, priority | Overseerr service configuration. `message`, `title`, and `priority` are optional. |

### ⚙️ Valid Values for Config Options

- `icon`: String, the name of the icon to display (e.g., `"desktop_computer"`).
- `topic`: String, the topic category for the notification.
- `priority`: Integer (1-5), sets the priority of the notification.
- `button`: Boolean (`true` or `false`), enables a button for the notification.
- `message`: String, the message content of the notification.
- `title`: String, the title of the notification.

# 👓 Endpoints Collection [/healthcheck]

## Health Check [GET]

+ Response 200 (application/json)

        {
            "status": 200
        }

## Test Overseerr Notification [POST]

Some description .

+ Request ( application/json )

    + Body

            {
                "notification_type": "overseerr",
                "subject": "test",
                "message": "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
                "image": "",
                "email": "",
                "username": "",
                "avatar": ""
            }

    + Schema

            {
              "type": "object",
              "properties": {
                    "notification_type": {
                      "type": "string"
                    },
                    "subject": {
                      "type": "string"
                    },
                    "message": {
                      "type": "string"
                    },
                    "image": {
                      "type": "string"
                    },
                    "email": {
                      "type": "string"
                    },
                    "username": {
                      "type": "string"
                    },
                    "avatar": {
                      "type": "string"
                    },
                }
            }

    + Headers

            Authorization: Basic dXNlcm5hbWU6cGFzc3dvcmQ=

+ Response 201 (application/json)

    + Headers

            Location: /overseerr

    + Body

            {

            }