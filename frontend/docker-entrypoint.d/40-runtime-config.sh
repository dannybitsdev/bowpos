#!/bin/sh
set -eu

api_url="${VITE_API_URL:-/api}"
escaped_api_url=$(printf '%s' "$api_url" | sed 's/\\/\\\\/g; s/"/\\"/g')
printf 'window.__BOWPOS_CONFIG__ = { apiUrl: "%s" };\n' "$escaped_api_url" > /usr/share/nginx/html/runtime-config.js