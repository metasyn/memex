.PHONY: serve downscale build deploy
.ONESHELL:

serve:
	python -m http.server

downscale:
	./downscale resources/img

build:
	cargo run build

deploy:
	./deploy.sh
