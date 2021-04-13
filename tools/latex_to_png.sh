#!/bin/bash

SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
xclip -o | "$SCRIPTPATH/pnglatex.git/pnglatex" -p amsmath -B Black -b Gray -d 600 -P 15x15 -o result.png