**NOTE: work in progress.**

# Isogeometric Analysis and Finite Element Method

The repo contains code to show how to implement FEM/IGA, coming from my dissertation (link below). Main topic is Isogeometric Analysis (IGA), but some code is there also for the Finite Element Method (FEM). The code was mostly written in 2009 for Matlab. In 2021 I improved it a bit and patched it to work with Octave, which seems to work very well. The repo contains:

* 2.3: code showing how to solve 1D problems using the Finite Element Method (FEM);
* 3.1: examples on how to draw curves and surfaces with implicit and parametric equations;
* 3.3: implementation of Bezier curves and surfaces with examples;
* 3.4: implementation of B-spline basis functions, curves and surfaces with examples;
* 3.5: implementation of NURBS basis functions, curves and surfaces with examples;
* 4.5: implementation of IGA on 1D problems with B-splines and NURBS, including h-refinement via knot insertion and degree elevation;
* 4.6: Gauss points and weights to speed up numerical quadrature for IGA;
* 4.7: implementation of IGA on 2D problems with numerical examples.

Note that the implementation is in no way optimized for speed. Computation of IGA solutions may take hours to complete and show the final result.

## Finite Element Method (FEM)

FEM is the main method used to numerically solve differential equations. The dissertation includes examples of solutions of 1D and 2D problems using this technique, but the 2D implementation uses Matlab/SimuLink, so it would need a specific implementation in Octave. 1D problems are showed instead in 2.3:

![eq](eq/eq_fem_h_refinement_1.png)

results in:

![example1](2.3/fem_h_refinement_1.svg.png)

## Curves and Surfaces with Implicit and Parametric Forms

Examples for drawing implicit and parametric equations:

![sphere_implicit](3.1/sphere_implicit.svg.png)

## Bezier Curves and Surfaces

Implementation of Bezier curves and surfaces:

![surf](3.3/bezier_surf.svg.png)

## B-spline Curves and Surfaces

Implementation of B-spline curves and surfaces:

![surf](3.4/bspline_surf_2.svg.png)
![surf](3.4/bspline_surf_ring.svg.png)

## NURBS Curves and Surfaces

Implementation of NURBS curves and surfaces:

![plate](3.5/nurbs_plate.svg.png)
![plate](3.5/nurbs_toroid.svg.png)

## Isogeometric Analysis with Knot Insertion

Chapter 4.5 includes algorithms for knot insertion, to refine the approximation:

![example5](4.5/iga_knot_insertion_circle.svg.png)

![example6](4.5/iga_knot_insertion_plate_hole.svg.png)

In chapter 4.6 I created weights and Gauss points to speed up numerical quadrature during the IGA process.

In chapter 4.7 there are more examples of solving 2D problems with IGA, (e.g.):

    ∇(∇u(x)) = 1, ∀x ∈ Ω
    u(x) = 1, ∀x ∈ Γ
    u(x) = 0, ∀x ∈ ∂Ω \ Γ

![solution](4.7/iga_2d_56_1.svg.png)

Refer to this [blog post](https://thebugfreeblog.blogspot.com/p/blog-page_17.html).

Link to dissertation: https://bugfreeblog.page.link/isogeometric-analysis-dissertation.

Link to presentation: https://bugfreeblog.page.link/isogeometric-analysis-presentation.