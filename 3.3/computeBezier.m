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
% along with this program.  If not, see <https://www.gnu.org/licenses/>.
%
% Copyright © 2008-2021 Luca Carlon

% This function computes the Bezier curve with control points in P
% on the domain u.
function [x, y, z] = computeBezier(P, u)
    % Determine the degree.
    n = length(P(:, 1));

    % Determinare the dimension.
    d = length(P(1, :));

    % Build Bernestein polynomials.
    B = inline('factorial(n)./(factorial(i).*factorial(n - i)).*u.^i.*(1 - u).^(n-i)', 'i', 'n', 'u');

    % Init.
    x = 0;
    y = 0;
    z = 0;

    % Compute the value.
    for i = 0:1:(n - 1)
        x = x + B(i, n - 1, u).*P(i + 1, 1);
        y = y + B(i, n - 1, u).*P(i + 1, 2);
        if d == 3, z = z + B(i, n - 1, u).*P(i + 1, 3); end
    end
endfunction