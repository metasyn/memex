.PHONY: deploy

.ONESHELL:

deploy:
		sshopts="ssh -o StrictHostKeyChecking=no -p 23"
		rsync --rsh="$$sshopts" -rvz --progress ./dist/* xander@metasyn.pw:/var/www/nginx/memex
