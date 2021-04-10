function [n, p, Xi, m, q, Eta, P] = defineBsplinePlateHole()
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