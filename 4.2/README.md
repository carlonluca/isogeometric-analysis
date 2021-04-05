### drawIGA1DExample

This script is an example of how to use IGA to approximate the solution of the PDE problem:

    u''(x) = 10
    u(0) = 0
    u(1) = 1

with increasing number of knots. Knot insertion results in finer mesh, converging to the exact solution (in red).

![example1](iga_knot_insertion_1.svg.png)

### drawIGA1DExample2

The second script solves the problem:

    1/50*u''(x) = x
    u(0) = 0
    u(1) = 1

using IGA, with know insertion.

![example2](iga_knot_insertion_2.svg.png)

### drawIGA1DExample3

The third script solved the problem:

    1/50*u''(x) = x
    u(0) = 0
    u(1) = 1

using degree 2 b-spline basis functions.

![example3](iga_knot_insertion_3.svg.png)