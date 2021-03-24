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

% dersBasisFuns computes the nonzero basis functions and
% their derivatives.
% Input:
%   i: index of the basis function to compute (i=0,1,...,n).
%   xi: point where the derivatives are evaluated;
%   p: degree of the basis functions;
%   n: defined accordingly to the knot vector;
%   Xi: knot vector.
% Output:
%   ders: bidimensional matrix where the element in position
%         (k,j) is the kth derivative of the function
%         N_{i-p+j,p} with 0<=k<=n and 0<=j<=p.
function ders = computeBsplineBasisDerivs(i, xi, p, n, Xi)
    % First evaluate the basis functions.
    ndu(1, 1) = 1;
    left(p+1) = 0;
    right(p+1) = 0;
    for j=1:p
        left(j+1) = xi-Xi(i+1-j+1);
        right(j+1) = Xi(i+j+1)-xi;
        saved = 0;
        for r=0:j-1
            ndu(j+1, r+1) = right(r+2)+left(j-r+1);
            temp = ndu(r+1, j)./ndu(j+1, r+1);
            ndu(r+1, j+1) = saved+right(r+2).*temp;
            saved = left(j-r+1).*temp;
        end
        ndu(j+1, j+1) = saved;
    end
    % Load the basis functions.
    ders(n+1, p+1) = 0;
    for j=1:p+1
        ders(1, j) = ndu(j, p+1);
    end
    % Evaluation of the derivatives.
    % Loop over function index.
    a(p, p) = 0;
    for r=0:p
        % Indices for the matrix containing the derivatives.
        s1 = 0; s2 = 1;
        a(1, 1) = 1;
        % Loop for the computation of the kth derivative.
        for k=1:n
            d = 0;
            rk = r-k; pk = p-k;
            if r >= k
                a(s2+1, 0+1) = a(s1+1, 0+1)./ndu(pk+2, rk+1);
                d = a(s2+1, 0+1).*ndu(rk+1, pk+1);
            end
            if rk >= -1, j1 = 1;
            else j1 = -rk; end;
            if r-1 <= pk, j2 = k-1;
            else j2 = p-r; end;
            for j=j1:j2
                a(s2+1,j+1) =...
                    (a(s1+1, j+1)-a(s1+1, j-1+1))./ndu(pk+1+1, rk+2);
                d = d+a(s2+1, j+1).*ndu(rk+j+1, pk+1);
            end
            if r <= pk
                a(s2+1, k+1) = -a(s1+1, k)./ndu(pk+2, r+1);
                d = d+a(s2+1, k+1).*ndu(r+1, pk+1);
            end
            ders(k+1, r+1) = d;
            j=s1; s1 = s2; s2 = j;
        end
    end
    % Multiply by the correct factors.
    r = p;
    for k=1:n
        for j=0:p
            ders(k+1, j+1) = ders(k+1, j+1).*r;
        end
        r = r.*(p-k);
    end
endfunction