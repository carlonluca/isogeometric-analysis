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

% Compute the nonvanishing basis functions.
% Input:
%   i: index of the basis function to compute (this value cannot be smaller
%       than p);
%   xi: value in which the basis function is being evaluated;
%   p: degree of the basis function;
%   Xi: knot vector over which the basis function is being built.
% Output:
%   N: vector containing the value of all the nonvanishing basis functions:
%   N(1)=N_{i-p},...,N(p+1)=N_{i}.
function N = computeBsplineBasisFuns(i, xi, p, Xi)
    % Preallocation.
    N = zeros(1, p+1);
    left = zeros(1, p+1);
    right = zeros(1, p+1);
    % Computation of the inverse triangular table starting from degree 0.
    N(1) = 1;
    for j = 1:p
        left(j+1) = xi-Xi(i+1-j+1);
        right(j+1) = Xi(i+j+1)-xi;
        saved = 0;
        for r = 0:j-1
            temp = N(r+1)./(right(r+1+1)+left(j-r+1));
            N(r+1) = saved+right(r+1+1).*temp;
            saved = left(j-r+1).*temp;
        end
        N(j+1) = saved;
    end
endfunction