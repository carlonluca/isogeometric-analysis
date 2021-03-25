%
% Project: Approximation and Finite Elements in Isogeometric Problems
% Author:  Luca Carlon
% Date:    2009.11.30
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

function [] = drawNURBSCurve(n, p, Xi, Pw)
    % Variable.
    xi = linspace(0, 1, 1000);

    hold on;
    axis equal;

    % Draw the curve.
    for i = 1:length(xi)
        C(i, :) = computeNURBSCurvePoint(n, p, Xi, Pw, xi(i));
    end
    plot(C(:, 1), C(:, 2), 'k', 'LineWidth', 2);

    % Compute the control points.
    P(:, 1:length(Pw(1, :))-1) = 0;
    for i = 1:length(Pw(:, 1))
        P(i, :) = Pw(i, 1:length(Pw(1, :))-1)./Pw(i, end);
    end

    % Draw the control points.
    for i = 1:length(P(:, 1))
        plot(P(i, 1), P(i, 2), '.', 'MarkerSize', 10, 'Color', 'red');
    end

    % Draw the lines.
    for i = 1:length(P(:, 1))-1
        plot([P(i, 1), P(i+1, 1)], [P(i, 2), P(i+1, 2)], '--', 'Color', 'k');
    end
endfunction