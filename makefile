SHELL=/bin/zsh

.PHONY: site
.ONESHELL:

clean:
	@echo
	@echo "🧹 Cleaning..."
	rm -rf ./dist

site: clean
	@echo
	@echo "🖨  Copying..."
	cp -r resources dist
