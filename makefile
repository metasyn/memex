SHELL=/bin/zsh

.PHONY: clean build
.ONESHELL:

clean:
	@echo
	@echo "🧹 Cleaning..."
	rm -rf ./dist

server:
	python3 -m http.server 8000

build:
	@echo
	@echo "🖨  Copying..."
	mkdir -p dist
	cp -r resources/* dist
