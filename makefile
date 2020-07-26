SHELL=/bin/zsh

.PHONY: clean build tar
.ONESHELL:

clean:
	@echo
	@echo "🧹 Cleaning..."
	rm -rf ./build ./dist
	mkdir build
	mkdir dist

build:
	@echo
	@echo "🖨  Copying..."
	cp -r resources build

tar: clean build
	tar -C build -czvf dist/site.tgz .

deploy:
	scp -P 23 dist/site.tgz xander@metasyn.pw:/tmp/site.tgz
	ssh -t -p 23 xander@metasyn.pw bash -c "cd /var/www/html; sudo cp /tmp/site.tgz . && sudo tar xvzf site.tgz && sudo rm site.tgz && rm /tmp/site.tgz"
