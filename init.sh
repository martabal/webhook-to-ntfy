#!/bin/sh

# Used for Docker

if [[ -f "/config/config.yml" ]]; then
    echo "$FILE exists." ;
    exec /app/webhookntfy ;
else
    mkdir -p /config;
    cp /app/config.example.yaml /config/config.yaml;
    exec /app/webhookntfy ;
fi