SHELL=/bin/zsh

.PHONY: server
.ONESHELL:

server:
	python3 -m http.server 8000
