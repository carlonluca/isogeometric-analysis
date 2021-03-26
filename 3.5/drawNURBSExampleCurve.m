%
% Project: Approximation and Finite Elements in Isogeometric Problems
% Author:  Luca Carlon
% Date:    2021.03.26
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

% Define the control points.
P = [0, 0; 1, 1; 2, 0.5; 3, 0.5; 0.5, 1.5; 1.5, 0];
% Define the knot vector.
Xi = [0, 0, 0, 0.25, 0.5, 0.75, 1, 1, 1];
% Define the degree.
p = 2;
% Define the weights.
w = ones(1, length(P(:, 1)));
% Compute the dimension.
d = length(P(1, :));
% Weighted control points.
Pw(:, 1) = P(:, 1).*w';
Pw(:, 2) = P(:, 2).*w';
Pw(:, 3) = w';
% Variable.
xi = linspace(0, 1, 1000);
% Deinition of the scalars.
n = 5;
p = 2;

subplot(2, 1, 1);
hold on;
axis([0, 3.5, 0, 2]);
grid on;
box on;
title('(a)');
xlabel('x');
ylabel('y');
drawNURBSCurve(n, p, Xi, Pw);

subplot(2, 1, 2);
hold on;
grid on;
box on;
title('(b)');
s = sprintf('R_{i}^{%d}, i=0,...,%d', p, n);
xlabel('\xi');
ylabel(s);
drawNURBSBasisFuns(n, p, Xi, w);