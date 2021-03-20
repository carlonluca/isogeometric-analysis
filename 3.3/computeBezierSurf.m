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

% Computes the value of the Bezier surface of control points P in
% the coordinates (u, v).

function [x, y, z] = computeBezierSurf(P, u, v)
    % Build Bernestein polynomials.
    B = inline('factorial(n)./(factorial(i).*factorial(n - i)).*u.^i.*(1 - u).^(n-i)', 'i', 'n', 'u');

    % Dimensions.
    n = size(P); n = n(2);
    m = size(P); m = m(3);

    % Init.
    x = 0;
    y = 0;
    z = 0;

    % Compute the value.
    for i = 0:1:(n - 1)
        for j = 0:1:(m - 1)
            x = x + B(i, n - 1, u).*B(j, m - 1, v).*P(1, i + 1, j + 1);
            y = y + B(i, n - 1, u).*B(j, m - 1, v).*P(2, i + 1, j + 1);
            z = z + B(i, n - 1, u).*B(j, m - 1, v).*P(3, i + 1, j + 1);
        end
    end
endfunction