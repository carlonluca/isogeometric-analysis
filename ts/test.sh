#!/bin/bash

tsc -p tsconfig.test.json
node js_test/test/matrix_test.js
node js_test/test/bspline_test.js
node js_test/test/nurbs_test.js
node js_test/test/lu_test.js
node js_test/test/linsystem_test.js
node js_test/test/quad_test.js
node js_test/test/bezier_test.js
