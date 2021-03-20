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

% derivsBasisFun determines the derivatives of the i-th B-spline basis
% function of degree p built over the knot vector Xi in the point xi of
% degree up to degree n.
% Input:
%   p: degree of the function;
%   Xi: knot vector;
%   i: index of the B-spline basis function to differentiate;
%   xi: point in which to evaliate the derivative;
%   n: max degree of derivative to find;
% Output:
%   derivs: derivs(k) contains the k-th-1 derivative of the function in xi.
function derivs = computeBsplineBasisDeriv(p, Xi, i, xi, n)
    % Check if xi is outside the domain.
    derivs = zeros(n+1);
    if xi < Xi(i+1) || xi >= Xi(i+p+1+1)
        for k = 0:n, derivs(k+1) = 0; end;
        return;
    end
    for j = 0:p
        if xi >= Xi(i+j+1) && xi < Xi(i+j+2), N(j+1, 1) = 1;
        else N(j+1, 1) = 0; end;
    end
    % Compute the triangle.
    for k = 1:p
        if N(0+1, k-1+1) == 0, saved = 0;
        else saved = ((xi-Xi(i+1)).*N(0+1, k-1+1))./(Xi(i+k+1)-Xi(i+1)); end;
        for j = 0:p-k+1-1
            Xileft = Xi(i+j+1+1);
            Xiright = Xi(i+j+k+1+1);
            if N(j+1+1, k-1+1) == 0
                N(j+1, k+1) = saved;
                saved = 0;
            else
                temp = N(j+1+1, k-1+1)./(Xiright-Xileft);
                N(j+1, k+1) = saved+(Xiright-xi).*temp;
                saved = (xi-Xileft).*temp;
            end
        end
    end
    for k = 1:n
        for j = 0:k, ND(j+1) = N(j+1, p-k+1); end;
        for jj = 1:k
            if ND(1) == 0, saved = 0;
            else saved = ND(0+1)./(Xi(i+p-k+jj+1)-Xi(i+1)); end;
            for j = 0:k-jj
                Xileft = Xi(i+j+2);
                Xiright = Xi(i+j+p+jj+1);
                if ND(j+1+1) == 0
                    ND(j+1) = (p-k+jj).*saved;
                    saved = 0;
                else
                    temp = ND(j+2)./(Xiright-Xileft);
                    ND(j+1) = (p-k+jj).*(saved-temp);
                    saved = temp;
                end
            end
        end
        derivs(k+1) = ND(1);
    end
endfunction