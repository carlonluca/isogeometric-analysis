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
Draws a b-spline surface.

    drawBsplineSurfDemo

![surf](bspline_surf.svg.png)