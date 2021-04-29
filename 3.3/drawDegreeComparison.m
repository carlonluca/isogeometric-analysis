%
% Project: Approximation and Finite Elements in Isogeometric Problems
% Author:  Luca Carlon
% Date:    2021.04.29
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

% Draws a Bezier curve with an increasing number of control points.

drawBezierCurve([0, 0; 1, 1], 1, hsv2rgb([0, 1, 1]), hsv2rgb([0, 1, 1]));
drawBezierCurve([0, 0; 1, 1; 2, 0.5], 1, hsv2rgb([1/5, 1, 1]), hsv2rgb([1/5, 1, 1]));
drawBezierCurve([0, 0; 1, 1; 2, 0.5; 3, 0.5], 1, hsv2rgb([2/5, 1, 1]), hsv2rgb([2/5, 1, 1]));
drawBezierCurve([0, 0; 1, 1; 2, 0.5; 3, 0.5; 0.5, 1.5], 1, hsv2rgb([3/5, 1, 1]), hsv2rgb([3/5, 1, 1]));
drawBezierCurve([0, 0; 1, 1; 2, 0.5; 3, 0.5; 0.5, 1.5; 1.5, 0], 1, hsv2rgb([4/5, 1, 1]), hsv2rgb([4/5, 1, 1]));