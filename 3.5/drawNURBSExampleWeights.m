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
% Variable.
xi = linspace(0, 1, 1000);
% Deinition of the scalars.
n = 5;
p = 2;

index = 1;
for i = 0.5:1.5:3.5
   % Define the weights.
   w = ones(1, length(P(:, 1)));
   w(2) = i;
   % Compute the dimension.
   d = length(P(1, :));
   % Weighted control points.
   Pw(:, 1) = P(:, 1).*w';
   Pw(:, 2) = P(:, 2).*w';
   Pw(:, 3) = w';
   plots(index++) = drawNURBSCurve(n, p, Xi, Pw, hsv2rgb([(i - 0.5)/4.5, 1, 1]), i != 3.5);
end

axis([0, 3, 0, 1.8])
title("NURBS with varying weights for a single control point")
h = legend(plots, "w(2) = 0.5", "w(2) = 2", "w(2) = 3.5");
legend(h, "location", "northeastoutside");