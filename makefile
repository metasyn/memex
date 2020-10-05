SHELL=/bin/zsh

.PHONY: build server
.ONESHELL:

build:
	nimble run memex build

server:
	python3 -m http.server 8000
