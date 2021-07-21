#!/bin/bash

sed -i 's/font-family="FreeSans"/font-family="Ubuntu"/g' "$1"
convert -background transparent -density $((72*8)) -resize $2 "$1" "$1.png"
