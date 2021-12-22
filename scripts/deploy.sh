#!/usr/bin/env bash
set -euo pipefail

sshopts="ssh -o StrictHostKeyChecking=no -p 23"
rsync --rsh="$sshopts" -zavhrc ./dist/* xander@metasyn.pw:/var/www/nginx/memex
