%
% Project: Approximation and Finite Elements in Isogeometric Problems
% Author:  Luca Carlon
% Date:    2009.10.25
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

function [] = drawNURBSSurf(n, p, Xi, m, q, Eta, Pw)

% Draw the physical domain.
hold on;
axis equal;
box on;
grid on;
xlabel('x');
ylabel('y');
zlabel('z');

% Define the sample values.
spacexi = linspace(0, 1, 100);
spaceeta = linspace(0, 1, 100);

% Define the points to be drawn.
for i = 1:length(spacexi)
    for j = 1:length(spaceeta)
        S = computeNURBSSurfPoint(n, p, Xi, m, q, Eta, Pw, spacexi(i), spaceeta(j));
        X(i, j) = S(1);
        Y(i, j) = S(2);
        Z(i, j) = S(3);
    end
end
surfc(X, Y, Z);
shading flat;
clear S;

% Draw the control points.
for i = 1:length(Pw(:, 1, 1))
    for j = 1:length(Pw(1, :, 1))
        plot3(Pw(i, j, 1)./Pw(i, j, 4), Pw(i, j, 2)./Pw(i, j, 4),...
            Pw(i, j, 3)./Pw(i, j, 4),...
            '.', 'MarkerSize', 10, 'Color', 'red');
    end
end

% Draw the lines.
for i = 1:length(Pw(:, 1, 1))-1
    for j = 1:length(Pw(1, :, 1))-1
        plot3([Pw(i, j, 1)./Pw(i, j, 4), Pw(i+1, j, 1)./Pw(i+1, j, 4)],...
            [Pw(i, j, 2)./Pw(i, j, 4), Pw(i+1, j, 2)./Pw(i+1, j, 4)],...
            [Pw(i, j, 3)./Pw(i, j, 4), Pw(i+1, j, 3)./Pw(i+1, j, 4)],...
            '--');
        plot3([Pw(i, j, 1)./Pw(i, j, 4), Pw(i, j+1, 1)./Pw(i, j+1, 4)],...
            [Pw(i, j, 2)./Pw(i, j, 4), Pw(i, j+1, 2)./Pw(i, j+1, 4)],...
            [Pw(i, j, 3)./Pw(i, j, 4), Pw(i, j+1, 3)./Pw(i, j+1, 4)],...
            '--');
    end
end

% Draw the isocurves on Eta.
for j = 1:length(Eta)
    for i = 1:length(spacexi)
        S(i, :) = computeNURBSSurfPoint(n, p, Xi, m, q, Eta, Pw, spacexi(i), Eta(j));
    end
    plot3(S(:, 1), S(:, 2), S(:, 3), 'k');
end

% Draw the isocurves on Xi.
for i = 1:length(Xi)
    for j = 1:length(spaceeta)
        S(j, :) = computeNURBSSurfPoint(n, p, Xi, m, q, Eta, Pw, Xi(i), spaceeta(j));
    end
    plot3(S(:, 1), S(:, 2), S(:, 3), 'k');
end