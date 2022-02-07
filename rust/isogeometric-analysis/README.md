# Isogeometric Analysis

This crate is a working in progress. The idea is that it should contain a trivial implementation of the algorithms involved in the approximation of partial differential equations through an isogeometric approach. The crate includes trivial implementations of Bezier, B-spline and NURBS curves and surfaces, numerical integration and basic linear algebra.

The crate is a work in progress and currently only implements:

* Bezier curves (direct method and De Casteljau's)
* Bezier surfaces (direct method and De Casteljau's)
* Rational Bezier curves

## Examples

Examples can be found in the documentation, in the unit tests and in two other crates:

* https://github.com/carlonluca/isogeometric-analysis/tree/master/rust/isogeometric-analysis-benchmarks
* https://github.com/carlonluca/isogeometric-analysis/tree/master/rust/isogeometric-analysis-demos

## Demo

The library includes procedures to load data for the Utah teapot, teacup and teaspoon: https://github.com/rm-hull/newell-teapot. This is a demo of how patches are computed by the crate and rendered through gnuplot.

![teapot](https://github.com/carlonluca/isogeometric-analysis/raw/master/rust/isogeometric-analysis/images/bezier_teapot.png)
![teapot](https://github.com/carlonluca/isogeometric-analysis/raw/master/rust/isogeometric-analysis/images/bezier_teacup.png)
![teapot](https://github.com/carlonluca/isogeometric-analysis/raw/master/rust/isogeometric-analysis/images/bezier_teaspoon.png)

## More Info

More info about Isogeometric Analysis and these topics can be downloaded from: https://bugfreeblog.duckdns.org/isogeometric-analysis.