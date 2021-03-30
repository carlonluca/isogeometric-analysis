%
% Project: Approximation and Finite Elements in Isogeometric Problems
% Author:  Luca Carlon
% Date:    2009.04.07
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

% Phi is a piecewise linear function centered in xj where the slope is
% m until 1.
% Input:
%    xj: abscissa where the function is supposed to be centered (where value
%        is 1).
%    m: angular coefficient.
%    x: variable.
% Output:
%    y: value of the function in x.

function y = computePhi(xj, m, x)
for i = 1:length(x)
    if x(i) >= xj - 1/m
        if x(i) <= xj
            y(i) = m*x(i)+1-m*xj;
            continue;
        end
    end
    if x(i) >= xj
        if x(i) <= xj + 1/m
            y(i) = -m*x(i)+1+m*xj;
            continue;
        end
    end
    y(i) = 0;
end