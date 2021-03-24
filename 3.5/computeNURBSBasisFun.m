%
% Project: Approximation and Finite Elements in Isogeometric Problems
% Author:  Luca Carlon
% Date:    2009.11.20
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

% NURBSBasisFun computes the derivatives of the i-th NURBS basis function.
% Input:
%   i: index of the NURBS basis function;
%   xi: point where to evaluate the derivative;
%   n: number of control points minus 1;
%   p: degree of the NURBS basis function;
%   Xi: knot vector over which the NURBS basis function has to be built;
%   w: vector of the weights.
% Output:
%   R: value of the derivative.
function R = computeNURBSBasisFun(i, xi, n, p, Xi, w)
    spanXi = findSpan(n, p, xi, Xi);
    if i < spanXi-p || i > spanXi, R = 0;
    else
        N = computeBsplineBasisFuns(spanXi, xi, p, Xi);
        R = N(p+1-(spanXi-i)).*w(i+1)./(N*w(spanXi-p+1:spanXi+1)');
    end
endfunction