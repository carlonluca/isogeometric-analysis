#!/bin/bash

convert -background transparent -density $2 -resize $2 "$1" "$1.png"
