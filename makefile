.PHONY: serve downscale build deploy
.ONESHELL:

serve:
	python3 -m http.server

downscale:
	./downscale resources/img

build:
	cargo run build

deploy:
	./deploy.sh
