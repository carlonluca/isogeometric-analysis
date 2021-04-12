%
% Project: Approximation and Finite Elements in Isogeometric Problems
% Author:  Luca Carlon
% Date:    2021.04.12
%
% Copyright (c) 2021 Luca Carlon. All rights reserved.
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

function [n, p, Xi, m, q, Eta, P, d, w] = defineNURBSToroid()
% Definition of the control points.
P(1, 1, :) = [5, 0, -1];
P(2, 1, :) = [5, 5, -1];
P(3, 1, :) = [0, 5, -1];
P(4, 1, :) = [-5, 5, -1];
P(5, 1, :) = [-5, 0, -1];
P(6, 1, :) = [-5, -5, -1];
P(7, 1, :) = [0, -5, -1];
P(8, 1, :) = [5, -5, -1];
P(9, 1, :) = [5, 0, -1];

P(1, 2, :) = [6, 0, -1];
P(2, 2, :) = [6, 6, -1];
P(3, 2, :) = [0, 6, -1];
P(4, 2, :) = [-6, 6, -1];
P(5, 2, :) = [-6, 0, -1];
P(6, 2, :) = [-6, -6, -1];
P(7, 2, :) = [0, -6, -1];
P(8, 2, :) = [6, -6, -1];
P(9, 2, :) = [6, 0, -1];

P(1, 3, :) = [6, 0, 0];
P(2, 3, :) = [6, 6, 0];
P(3, 3, :) = [0, 6, 0];
P(4, 3, :) = [-6, 6, 0];
P(5, 3, :) = [-6, 0, 0];
P(6, 3, :) = [-6, -6, 0];
P(7, 3, :) = [0, -6, 0];
P(8, 3, :) = [6, -6, 0];
P(9, 3, :) = [6, 0, 0];

P(1, 4, :) = [6, 0, 1];
P(2, 4, :) = [6, 6, 1];
P(3, 4, :) = [0, 6, 1];
P(4, 4, :) = [-6, 6, 1];
P(5, 4, :) = [-6, 0, 1];
P(6, 4, :) = [-6, -6, 1];
P(7, 4, :) = [0, -6, 1];
P(8, 4, :) = [6, -6, 1];
P(9, 4, :) = [6, 0, 1];

P(1, 5, :) = [5, 0, 1];
P(2, 5, :) = [5, 5, 1];
P(3, 5, :) = [0, 5, 1];
P(4, 5, :) = [-5, 5, 1];
P(5, 5, :) = [-5, 0, 1];
P(6, 5, :) = [-5, -5, 1];
P(7, 5, :) = [0, -5, 1];
P(8, 5, :) = [5, -5, 1];
P(9, 5, :) = [5, 0, 1];

P(1, 6, :) = [4, 0, 1];
P(2, 6, :) = [4, 4, 1];
P(3, 6, :) = [0, 4, 1];
P(4, 6, :) = [-4, 4, 1];
P(5, 6, :) = [-4, 0, 1];
P(6, 6, :) = [-4, -4, 1];
P(7, 6, :) = [0, -4, 1];
P(8, 6, :) = [4, -4, 1];
P(9, 6, :) = [4, 0, 1];

P(1, 7, :) = [4, 0, 0];
P(2, 7, :) = [4, 4, 0];
P(3, 7, :) = [0, 4, 0];
P(4, 7, :) = [-4, 4, 0];
P(5, 7, :) = [-4, 0, 0];
P(6, 7, :) = [-4, -4, 0];
P(7, 7, :) = [0, -4, 0];
P(8, 7, :) = [4, -4, 0];
P(9, 7, :) = [4, 0, 0];

P(1, 8, :) = [4, 0, -1];
P(2, 8, :) = [4, 4, -1];
P(3, 8, :) = [0, 4, -1];
P(4, 8, :) = [-4, 4, -1];
P(5, 8, :) = [-4, 0, -1];
P(6, 8, :) = [-4, -4, -1];
P(7, 8, :) = [0, -4, -1];
P(8, 8, :) = [4, -4, -1];
P(9, 8, :) = [4, 0, -1];

P(1, 9, :) = [5, 0, -1];
P(2, 9, :) = [5, 5, -1];
P(3, 9, :) = [0, 5, -1];
P(4, 9, :) = [-5, 5, -1];
P(5, 9, :) = [-5, 0, -1];
P(6, 9, :) = [-5, -5, -1];
P(7, 9, :) = [0, -5, -1];
P(8, 9, :) = [5, -5, -1];
P(9, 9, :) = [5, 0, -1];

d = length(P(1, 1, :));

% Define the knot vectors.
Xi = [0, 0, 0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1, 1, 1];
Xi = Xi.*4;
Eta = [0, 0, 0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1, 1, 1];
Eta = Eta.*4;

% Define the scalars.
n = 8;
p = 2;
m = 8;
q = 2;

w1 = [1; 1/sqrt(2); 1; 1/sqrt(2); 1; 1/sqrt(2); 1; 1/sqrt(2); 1]
w2 = [1/sqrt(2); 1/2; 1/sqrt(2); 1/2; 1/sqrt(2); 1/2; 1/sqrt(2); 1/2; 1/sqrt(2);]
w = ones(9, 9)
w(:, 1) = w1;
w(:, 2) = w2;
w(:, 3) = w1;
w(:, 4) = w2;
w(:, 5) = w1;
w(:, 6) = w2;
w(:, 7) = w1;
w(:, 8) = w2;
w(:, 9) = w1;