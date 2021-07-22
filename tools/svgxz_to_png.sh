#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
unxz -k "$1.xz"
"$SCRIPT_DIR/svg_to_png.sh" "$1" $2
rm "$1"