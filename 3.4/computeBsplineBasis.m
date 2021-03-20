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

% Evaluates the value of the i-th B-spline basis function of degree p over
% the knot vector Xi in xi.
% Input:
%   p: degree of the function to evaluate;
%   m: number of knots - 1;
%   Xi: knot vector over which the basis function is built;
%   i: index of the B-spline basis function to compute;
%   xi: point where to evaluate the B-spline basis function.
% Output:
%   Nip: value of the B-spline basis function in xi.
function Nip = computeBsplineBasis(p, m, Xi, i, xi)
    % Check to see if we're evaluating the first or the last basis function at
    % the beginning or at the end of the knot vector.
    if (i == 0 && xi == Xi(0+1)) || (i == m-p-1 && xi == Xi(m+1))
        Nip = 1; return;
    end
    % When xi is out of the domain it is set to zero.
    if (xi < Xi(i+1) || xi >= Xi(i+p+1+1))
        Nip = 0; return;
    end
    % Preallocation and computation of the temparary values of the functions to
    % be used according to the triangular table.
    N = zeros(p+1);
    for j = 0:p
        if xi >= Xi(i+j+1) && xi < Xi(i+j+1+1), N(j+1) = 1;
        else N(j+1) = 0; end;
    end
    % Computation of the rest of the triangular table.
    for k = 1:p
        if N(1) == 0, saved = 0;
        else saved = ((xi-Xi(i+1)).*N(0+1))./(Xi(i+k+1)-Xi(i+1)); end;
        for j = 0:p-k+1-1
            Xileft = Xi(i+j+1+1);
            Xiright = Xi(i+j+k+1+1);
            if N(j+1+1) == 0
                N(j+1) = saved;
                saved = 0;
            else
                temp = N(j+1+1)./(Xiright-Xileft);
                N(j+1) = saved+(Xiright-xi).*temp;
                saved = (xi-Xileft).*temp;
            end
        end
    end
    Nip = N(1);
endfunction