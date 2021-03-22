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

function [] = drawBsplineSurf(n, p, Xi, m, q, Eta, P, varargin)
    % Draw the physical domain.
    hold on;
    box on;
    grid on;
    colormap('jet');
    xlabel('x');
    ylabel('y');
    zlabel('z');

    if nargin == 8
        Q = P;
        Q(:, :, 3) = varargin{1};
    end

    % Define the sample values.
    spacexi = linspace(Xi(1), Xi(end), 100);
    spaceeta = linspace(Eta(1), Eta(end), 100);

    % Define the points to be drawn.
    C = zeros(length(spacexi), length(spaceeta));
    X = C;
    Y = C;
    Z = C;
    for i = 1:length(spacexi)
        for j = 1:length(spaceeta)
            S = computeBsplineSurfPoint(n, p, Xi,...
                m, q, Eta, P, spacexi(i), spaceeta(j));
            X(i, j) = S(1);
            Y(i, j) = S(2);
            Z(i, j) = S(3);
            if nargin == 8
                T = computeBsplineSurfPoint(n, p, Xi,...
                    m, q, Eta, Q, spacexi(i), spaceeta(j));
                C(i, j) = T(3);
            end
        end
    end
    if nargin == 8
        surf(X, Y, Z, C);
    else
        surf(X, Y, Z);
    end
    shading interp;
    clear S;

    % Draw the control points.
    for i = 1:length(P(:, 1, 1))
        for j = 1:length(P(1, :, 1))
            plot3(P(i, j, 1), P(i, j, 2),...
                P(i, j, 3),...
                '.', 'MarkerSize', 10, 'Color', 'red');
        end
    end

    % Draw the lines.
    for i = 1:length(P(:, 1, 1))
        for j = 1:length(P(1, :, 1))
            if i+1<=length(P(:, 1, 1))
                plot3([P(i, j, 1), P(i+1, j, 1)],...
                    [P(i, j, 2), P(i+1, j, 2)],...
                    [P(i, j, 3), P(i+1, j, 3)],...
                    '--');
            end
            if j+1<=length(P(1, :, 1))
                plot3([P(i, j, 1), P(i, j+1, 1)],...
                    [P(i, j, 2), P(i, j+1, 2)],...
                    [P(i, j, 3), P(i, j+1, 3)],...
                    '--');
            end
        end
    end

    % Draw the isocurves on Eta.
    for j = 1:length(Eta)
        for i = 1:length(spacexi)
            S(i, :) = computeBsplineSurfPoint(n, p, Xi, m, q, Eta, P, spacexi(i), Eta(j));
        end
        plot3(S(:, 1), S(:, 2), S(:, 3), 'k');
    end

    % Draw the isocurves on Xi.
    for i = 1:length(Xi)
        for j = 1:length(spaceeta)
            S(j, :) = computeBsplineSurfPoint(n, p, Xi, m, q, Eta, P, Xi(i), spaceeta(j));
        end
        plot3(S(:, 1), S(:, 2), S(:, 3), 'k');
    end

    xlim ([-3, 0]);
    ylim ([0, 3]);
endfunction