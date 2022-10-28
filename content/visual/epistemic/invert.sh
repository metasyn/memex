#!/usr/bin/env bash

set -euo pipefail
rm ./*white.png || true
for file in *.png; do
	printf "Inverting %s\n" "$file"
	name=$(basename "$file" .png)
	convert "$file" -channel RGB -negate "${name}_white.png"
	cp *.png ../../../resources/img/
done
