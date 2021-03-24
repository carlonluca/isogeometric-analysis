%
% Project: Approximation and Finite Elements in Isogeometric Problems
% Author:  Luca Carlon
% Date:    2009.09.09
%
% Copyright (c) 2009-2021 Luca Carlon. All rights reserved.
%
% This program is free software: you can redistribute it and/or modify
% it under the terms of the GNU General Public License as published by
% the Free Software Foundation, either version 3 of the License, or
% (at your option) any later version.
%
% This program is distributed in the hope that it will be useful,
% but WITHOUT ANY WARRANTY; without even the implied warranty of
% MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
% GNU General Public License for more details.
%
% You should have received a copy of the GNU General Public License
% along with this program. If not, see <http://www.gnu.org/licenses/>.
%

% computeNURBSBasisDeriv computes the value of the derivative i-th NURBS basis function
% in the point xi.
% Input:
%   i: index of the NURBS basis function to compute;
%   xi: point in which to compute the NURBS basis function;
%   p: degree of the NURBS basis function;
%   Xi: knot vector over which the NURBS basis function have to be
%       computed.
%   w: vector containing the weights.
% Output:
%   R: value of the i-th NURBS basis function computed in xi.
function R = computeNURBSBasisDeriv(i, xi, p, Xi, w)
    n = length(Xi)-p-2;
    spanXi = findSpan(n, p, xi, Xi);
    % Computation of all the nonvanishing B-spline basis functions in xi.
    Nips = computeBsplineBasisFuns(spanXi, xi, p, Xi);
    % Computation of the value for the denominator of the NURBS basis function.
    W = Nips*w(spanXi-p+1:spanXi+1)';
    % Compitation of the derivaives of the B-spline basis functions.
    dNips = computeBsplineBasisDerivs(spanXi, xi, p, 1, Xi);
    % Linear combination of the derivatives.
    Cw = w(spanXi-p+1:spanXi+1)*dNips(2, :)';
    % Values of the i-th basis function and relative derivative.
    Nip = computeBsplineBasis(p, n+p+1, Xi, i, xi);
    dNip = computeBsplineBasisDeriv(p, Xi, i, xi, 1);
    % Final result.
    R = (w(i+1).*dNip(1+1).*W - Nip.*w(i+1).*Cw)./(W.^2);
endfunction