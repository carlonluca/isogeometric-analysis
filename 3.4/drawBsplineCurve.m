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

% Draws a b-spline curve.
% Input:
%   n: scalar that indicates that n+1 is the number of control points;
%   p: degree of the curve;
%   Xi: knot vector;
%   P: control points where P(i, j) indicates the j-th coordinate of the
%       i-th control point.
function [] = drawBsplineCurve(n, p, Xi, P)
    hold on;
    axis equal;
    box on;
    grid on;
    xlabel('x');
    ylabel('y');
    % Define the sample values.
    spacexi = linspace(Xi(1), Xi(end), 100);

    % Define the points to be drawn.
    for i = 1:length(spacexi)
        C(i, :) = computeBsplineCurvePoint(n, p, Xi, P, spacexi(i));
    end
    if length(C(1, :)) == 3, plot3(C(:, 1), C(:, 2), C(:, 3), 'Color', 'black');
    else plot(C(:, 1), C(:, 2), 'Color', 'black'); end;

    % Draw the control points.
    for i = 1:length(P(:, 1))
        if length(P(1, :)) == 3
            plot3(P(i, 1), P(i, 2),...
                P(i, 3),...
                '.', 'MarkerSize', 15, 'Color', 'red');
        else
            plot(P(i, 1), P(i, 2),...
                '.', 'MarkerSize', 15, 'Color', 'red');
        end
    end

    % Draw the lines.
    for i = 1:length(P(:, 1))
        if i+1<=length(P(:, 1))
            if length(P(1, :)) == 3
                plot3([P(i, 1), P(i+1, 1)],...
                    [P(i, 2), P(i+1, 2)],...
                    [P(i, 3), P(i+1, 3)],...
                    '--');
            else
                plot([P(i, 1), P(i+1, 1)],...
                    [P(i, 2), P(i+1, 2)],...
                    '--');
            end
        end
    end
endfunction