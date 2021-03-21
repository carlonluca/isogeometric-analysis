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

% findSpan finds in which knot span a specific
% value of xi is to be found.
% Input:
%   n: max index of the control points (n+1 control points);
%   p: degree of the B-spline basis functions;
%   xi: scalar value to be found;
%   Xi: open knot vector {xi_0,...,xi_n,...,x_{n+p+1}}.
% Output:
%   i: knot span [xi_i, xi_{i+1}) in which u lies.
function i = findSpan(n, p, xi, Xi)
    % Special case.
    if xi == Xi(n+2), i = n; return; end;
    % Binary search.
    low = p;
    high = n+1;
    i = floor((low + high)./2);
    while xi < Xi(i+1) || xi >= Xi(i+2)
        if xi < Xi(i+1); high = i;
        else low = i; end;
        i = floor((low + high)./2);
    end
endfunction