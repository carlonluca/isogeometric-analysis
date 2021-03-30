%
% Project: Approximation and Finite Elements in Isogeometric Problems
% Author:  Luca Carlon
% Date:    2009.11.06
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

function [S, F, u] = computeIGA1DBsplines(a_1, a_0, f, xi_a, xi_b, Xi, p)
a = Xi(1);
b = Xi(end);
m = length(Xi) - 1;
for i = 1:(m-p-2)
    for j = 1:(m-p-2)
        integrand = @(y) stiffnessIntegral(a_1, p, Xi, i, j, y);
        S(i, j) = quad(integrand, a, b);
    end
    integrand = @(y) forceIntegral(a_1, f, xi_a, xi_b, p, Xi, i, y);
    F(i, 1) = quad(integrand, a, b);
end
u = linsolve(S, F);

function s = stiffnessIntegral(a_1, p, Xi, i, j, y)
for k = 1:length(y)
    derivs1 = computeBsplineBasisDeriv(p, Xi, i, y(k), 1);
    derivs2 = computeBsplineBasisDeriv(p, Xi, j, y(k), 1);
    s(k) = a_1(y(k)).*derivs1(2).*derivs2(2);
end

function force = forceIntegral(a_1, f, xi_a, xi_b, p, Xi, i, y)
for k = 1:length(y)
    derivs = computeBsplineBasisDeriv(p, Xi, i, y(k), 1);
    N = computeBsplineBasis(p, length(Xi)-1, Xi, i, y(k));
    force(k) = f(y(k)).*N -...
        a_1(y(k)).*dgamma(xi_a, xi_b, p, Xi, y(k)).*derivs(2);
end

function dg = dgamma(xi_a, xi_b, p, Xi, y)
for k = 1:length(y)
    derivs1 = computeBsplineBasisDeriv(p, Xi, 0, y(k), 1);
    derivs2 = computeBsplineBasisDeriv(p, Xi, length(Xi)-p-2, y(k), 1);
    dg(k) = xi_a.*derivs1(2) + xi_b.*derivs2(2);
end