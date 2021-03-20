%
% Project: Approximation and Finite Elements in Isogeometric Problems
% Author:  Luca Carlon
% Date:    2009.03.10
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

P(:, 1, 1) = [-3, 0, 1*2];
P(:, 1, 2) = [-2, 0, 3*2];
P(:, 1, 3) = [-1, 0, 3.5*2];
P(:, 1, 4) = [0, 0, 1*2];
P(:, 2, 1) = [-3, 1, 2];
P(:, 2, 2) = [-2, 1, 4];
P(:, 2, 3) = [-1, 1, 5];
P(:, 2, 4) = [0, 1, 2.5];
P(:, 3, 1) = [-3, 3, 0];
P(:, 3, 2) = [-2, 3, 2.5];
P(:, 3, 3) = [-1, 3, 4.5];
P(:, 3, 4) = [0, 3, 6.5];

% Draw the Bezier surface.
u = 0:0.05:1;
v = 0:0.05:1;

for i = 1:1:length(u(1, :))
    for j = 1:1:length(v(1, :))
        [x, y, z] = computeBezierSurf(P, u(1, i), v(1, j));
        X(i, j) = x;
        Y(i, j) = y;
        Z(i, j) = z;
    end
end

surfc(X, Y, Z);

% Blocks the plot.
hold on;

% Draw the lines.
for i = 1:1:(length(P(1, 1, :)) - 1)
    for j = 1:1:length(P(1, :, 1))
        plot3([P(1, j, i), P(1, j, i+1)], [P(2, j, i), P(2, j, i+1)], [P(3, j, i), P(3, j, i+1)], 'Color', [1, 0, 0], 'LineStyle', '-', 'LineWidth', 3);
    end
end
for i = 1:1:length(P(1, 1, :))
    for j = 1:1:(length(P(1, :, 1)) - 1)
        plot3([P(1, j, i), P(1, j+1, i)], [P(2, j, i), P(2, j+1, i)], [P(3, j, i), P(3, j+1, i)], 'Color', [1, 0, 0], 'LineStyle', '-', 'LineWidth', 2);
    end
end

% Draw points.
for i = 1:1:length(P(1, :, 1))
    for j = 1:1:length(P(1, 1, :))
        plot3(P(1, i, j), P(2, i, j), P(3, i, j), 'o', 'Color', 'k');
    end
end

xlabel('x');
ylabel('y');
zlabel('z');
colorbar;