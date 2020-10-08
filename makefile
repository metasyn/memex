SHELL=/bin/zsh

.PHONY: build server
.ONESHELL:
HOSTNAME := $(shell hostname -I)

build:
	nimble run memex build

server:
	echo ${HOSTNAME}
	echo ${HOSTNAME} | clip.exe
	python3 -m http.server
