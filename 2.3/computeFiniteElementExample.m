%
% Project: Approximation and Finite Elements in Isogeometric Problems
% Author:  Luca Carlon
% Date:    2009.04.07
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

function [S, F, u] = computeFiniteElementExample(alfa, sigma, f, domain, ua, ub, n)

% Definition of the domain.
a = domain(1);
b = domain(2);
m = 1/((b-a)/(n+1));

% Build the matrices.
S = zeros(n);
F = zeros(n, 1);

% Definition of the elements.
xx = linspace(a, b, n+2);

% Definition of the Dirichlet lift.
dgamma = @(x) (ua.*computeDphi(xx(1), m, x) + ub.*computeDphi(xx(end), m, x));

% Assembling the matrices.
for i = 2:(n+1)
    for j = 2:(n+1)
        integrand = @(x) (alfa(x).*computeDphi(xx(i), m, x).*computeDphi(xx(j), m, x));
        S(i-1, j-1) = S(i-1, j-1) + quad(integrand, a, b);
    end

    integrand = @(x) (f(x).*computePhi(xx(i), m, x) -...
        alfa(x).*dgamma(x).*computeDphi(xx(i), m, x));
    F(i-1, 1) = F(i-1, 1) + quad(integrand, a, b);
end

u = linsolve(S, F);
u = [0; u; 0];