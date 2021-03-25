%
% Project: Approximation and Finite Elements in Isogeometric Problems
% Author:  Luca Carlon
% Date:    2009.11.20
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

% computeNURBSCurvePoint evaluates a NURBS curve in the specified
% point xi.
% Input:
%   n: defined accordingly to the knot vector;
%   p: degree of the NURBS curve;
%   Xi: knot vector on which the NURBS is defined;
%   Pw: weighted control points written in rows;
%   xi: point in which the curve has to be evaluated.
% Output:
%   C: value of the curve in the point xi.
function [C] = computeNURBSCurvePoint(n, p, Xi, Pw, xi)
   span = findSpan(n, p, xi, Xi);
   N = computeBsplineBasisFuns(span, xi, p, Xi);
   d = length(Pw(1, :));

   Cw = zeros(1, d);
   for j = 0:p
      Cw(1:d) = Cw(1:d) + N(j+1).*Pw(span-p+j+1, 1:d);
   end
   C(1:d) = Cw(1:d)./Cw(d);
endfunction