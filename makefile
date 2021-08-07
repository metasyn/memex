.PHONY: serve downscale build deploy

.ONESHELL:

serve:
		python -m http.server

downscale:
	./downscale resources/img

build:
	cargo run build

deploy:
		sshopts="ssh -o StrictHostKeyChecking=no -p 23"
		rsync --rsh="$$sshopts" -zavhrc ./dist/* xander@metasyn.pw:/var/www/nginx/memex
