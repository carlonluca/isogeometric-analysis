## B-Spline
### drawBsplineBasisFuns
![basis](bspline_basis.svg.png)
### drawBsplineBasisDerivs
Computes and draws b-spline basis functions and their derivatives.

    drawBsplineBasisDerivs([0, 0, 0, 0, 1, 4, 6, 8, 8, 8, 8],3);

![basis](bspline_basis_derivs.svg.png)

    drawBsplineBasisDerivs([0, 0, 0, 1, 2, 3, 4, 4, 5, 5, 5],2);

![basis](bspline_basis_derivs_2.svg.png)

### drawBsplineCurve
Draws a b-spline curve.

    drawBsplineCurve(5, 2, [0, 0, 0, 0.25, 0.5, 0.75, 1, 1, 1], [0, 0; 1, 1; 2, 0.5; 3, 0.5; 0.5, 1.5; 1.5, 0]);

![basis](bspline_curve.svg.png)

    drawBsplineCurve(5, 2, [0, 0, 0, 0.25, 0.5, 0.75, 1, 1, 1], [0, 0, 0; 1, 1, 1; 2, 0.5, 0; 3, 0.5, 0; 0.5, 1.5, 0; 1.5, 0, 1]);

![basis](bspline_curve_3d.svg.png)

### drawBivariateBsplineBasisFuns
Draws bivariate basis functions.

    drawBivariateBsplineBasisFuns([0, 0, 0, 0.5, 1, 1, 1], 3, 2, [0, 0, 0, 0.5, 1, 1, 1], 3, 2);

![basis](bspline_bivariate_basis.svg.png)

### drawBsplineSurf
Draws a b-spline surface. In the first demo, the basis functions are linear in both directions. In the second example, the same surface is drawn, but basis functions are quadratic in one direction.

    drawBsplineSurfDemo

![surf](bspline_surf.svg.png)

    drawBsplineSurfDemo2

![surf](bspline_surf_2.svg.png)

### drawBsplineRing

Draws a ring using the implemented B-spline algorithms.

![surf](bspline_surf_ring.svg.png)