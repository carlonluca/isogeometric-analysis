#!/bin/bash

SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
xclip -o | "$SCRIPTPATH/pnglatex.git/pnglatex" -p amsmath -F Black -b White -d 150 -P 15x15 -o result.png
