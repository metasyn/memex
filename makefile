.PHONY: serve downscale build deploy
.ONESHELL:

serve:
	python3 -m http.server

downscale:
	./scripts/downscale.sh resources/img

build:
	cargo run build

deploy:
	git push origin main
	git push github main
	./scripts/deploy.sh
