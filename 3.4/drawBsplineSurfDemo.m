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

% Demonstration of the drawBsplineSurf function.

% Control points.
P(1, 1, :) = [-3, 0, 1*2];
P(1, 2, :) = [-2, 0, 3*2];
P(1, 3, :) = [-1, 0, 3.5*2];
P(1, 4, :) = [0, 0, 1*2];
P(2, 1, :) = [-3, 1, 2];
P(2, 2, :) = [-2, 1, 4];
P(2, 3, :) = [-1, 1, 5];
P(2, 4, :) = [0, 1, 2.5];
P(3, 1, :) = [-3, 3, 0];
P(3, 2, :) = [-2, 3, 2.5];
P(3, 3, :) = [-1, 3, 4.5];
P(3, 4, :) = [0, 3, 6.5];

% Define the degrees (P(:, i, j)).
p = 1;
q = 1;

n = 2;
m = 3;

% Define the knot vectors.
Xi = [0, 0, 0.5, 1, 1];
Eta = [0, 0, 0.3, 0.6, 1, 1];

drawBsplineSurf(n, p, Xi, m, q, Eta, P);