#!/bin/bash

tsc -p tsconfig.test.json
node js_test/test/matrix_test.js
