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

% computeKnotInsertion created a new NURBS curve from an existing one in which a
% new knot barxi is inserted in [xi_k, xi_{k+1}) with multiplicity r, where
% barxi has currently multiplicity s.
% Input:
%   nP: defined accordingly to the knot vector Xi;
%   p: degree of the initial curve;
%   Xi: initial knot vector;
%   Pw: initial weighted control points;
%   barxi: value of the knot to be inserted;
%   k: index of the knot span where the knot has to be inserted;
%   s: initial multiplicity of the knot;
%   r: multiplicity of the knot in the new curve.
% Output:
%   nBarP: new value of n defined on the new knot vector;
%   barXi: new knot vector;
%   barPw: new weighted control points.
function [nBarP, barXi, barPw] = computeKnotInsertion(nP, p, Xi, Pw, barxi, k, s, r)
mP = nP+p+1;
nBarP = nP+r;
% Load the new knot vector.
barXi(1:k+1) = Xi(1:k+1);
barXi(k+1+1:k+r+1) = barxi;
barXi(k+1+r+1:mP+r+1) = Xi(k+1+1:mP+1);
% Save unaltered control points.
barPw(1:k-p+1, :) = Pw(1:k-p+1, :);
barPw(k-s+r+1:nP+r+1, :) = Pw(k-s+1:nP+1, :);
Rw(1:p-s+1, :) = Pw(k-p+1:k-s+1, :);
% Insert the knot r times.
for j = 1:r
    L = k-p+j;
    for i = 0:p-j-s
        alpha = (barxi-Xi(L+i+1))./(Xi(i+k+1+1)-Xi(L+i+1));
        Rw(i+1, :) = alpha.*Rw(i+1+1, :)+(1-alpha).*Rw(i+1, :);
    end
    barPw(L+1, :) = Rw(1, :);
    barPw(k+r-j-s+1, :) = Rw(p-j-s+1, :);
end
% Load remaining control points.
for i = L+1:k-s-1
    barPw(i+1, :) = Rw(i-L+1, :);
end