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

% bsplineCurvePoint determines the value of the B-spline curve in the
% point xi of the parametric space.
% Input:
%   n: scalar that indicates that n+1 is the number of control points;
%   p: degree of the cure;
%   Xi: knot vector;
%   P: control points where P(i, j) indicates the j-th coordinate of the
%       i-th control point.
%   xi: point of the parametric space where to evalute the curve.
% Output:
%   C: (xi, eta) is the value of the curve in the two directions.
function C = computeBsplineCurvePoint(n, p, Xi, P, xi)
    % Find the span in which xi lies.
    span = findSpan(n, p, xi, Xi);
    % Determine all the nonvanishing B-spline basis functions in xi.
    N = computeBsplineBasisFuns(span, xi, p, Xi);
    % Calculation of the value of the curve.
    if length(P(1, :)) == 3, C(3) = 0; else C(2) = 0; end;
    for i = 0:p
        C(1) = C(1)+N(i+1).*P(span-p+i+1, 1);
        C(2) = C(2)+N(i+1).*P(span-p+i+1, 2);
        if length(P(1, :)) == 3
            C(3) = C(3)+N(i+1).*P(span-p+i+1, 3);
        end
    end
endfunction