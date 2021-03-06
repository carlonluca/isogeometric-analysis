%
% Project: Approximation and Finite Elements in Isogeometric Problems
% Author:  Luca Carlon
% Date:    2009.10.13
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

% Returns data needed to draw a rectangular plate with a hole.
function [n, p, Xi, m, q, Eta, P, d, w] = defineNURBSPlateHole()

% Definition of the physical domain (plate with hole).
P(1, 1, :) = [-1, 0, 0];
P(2, 1, :) = [-1, sqrt(2)-1, 0];
P(3, 1, :) = [1-sqrt(2), 1, 0];
P(4, 1, :) = [0, 1, 0];

P(1, 2, :) = [-2.5, 0, 0];
P(2, 2, :) = [-2.5, 0.75, 0];
P(3, 2, :) = [-0.75, 2.5, 0];
P(4, 2, :) = [0, 2.5, 0];

P(1, 3, :) = [-4, 0, 0];
P(2, 3, :) = [-4, 4, 0];
P(3, 3, :) = [-4, 4, 0];
P(4, 3, :) = [0, 4, 0];

d = length(P(1, 1, :));

% Define the knot vectors.
Xi = [0, 0, 0, 0.5, 1, 1, 1];
Eta = [0, 0, 0, 1, 1, 1];

% Define the scalars.
n = 3;
p = 2;
m = 2;
q = 2;
w = ones(4, 3);