.PHONY: deploy

.ONESHELL:

serve:
		python -m http.server

deploy:
		sshopts="ssh -o StrictHostKeyChecking=no -p 23"
		rsync --rsh="$$sshopts" -zavhr ./dist/* xander@metasyn.pw:/var/www/nginx/memex
