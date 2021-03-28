%
% Project: Approximation and Finite Elements in Isogeometric Problems
% Author:  Luca Carlon
% Date:    2009.08.14
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

% computeNURBSSurfPoint evaluates a NURBS surface on a point (xi, eta)
% of the domain.
% Input:
%   n: defined acordingly to the knot vector Xi;
%   p: degree in the first direction;
%   Xi: knot vector in the first direction;
%   m: defined accordingly to the knot vector Eta;
%   q: degree in the second direction;
%   Eta: knot vector in the second direction;
%   Pw: weighted points written in the format P(xi, x_k, eta);
%   xi: coordinate in the first direction on which the
%       surface is being evaluated;
%   eta: coordinate in the second direction on which the
%        surface is being evaluated.
% Output:
%   S: value of the surface in the point (xi, eta).
function [S] = computeNURBSSurfPoint(n, p, Xi, m, q, Eta, Pw, xi, eta)
xiSpan = findSpan(n, p, xi, Xi);
etaSpan = findSpan(m, q, eta, Eta);
Nxi = computeBsplineBasisFuns(xiSpan, xi, p, Xi);
Neta = computeBsplineBasisFuns(etaSpan, eta, q, Eta);
d = length(Pw(1, 1, :));

Sw(d) = 0;
for i = 1:d
    Sw(i) = Nxi*Pw(xiSpan-p+1:xiSpan+1, etaSpan-q+1:etaSpan+1, i)*Neta';
end
S = Sw(1:d-1)./Sw(d);

% Piegl and Tiller version.
% temp = zeros(p+1, d);
% for l = 0:q
%     temp(l+1, 1:d) = 0;
%     for k = 0:p
%         xiSpan-p+k+1
%         etaSpan-q+1+1
%         Pw(xiSpan-p+k+1, :, etaSpan-q+1+1)
%         temp(l+1, 1:d) = temp(l+1, 1:d) +...
%             Nxi(k+1).*Pw(xiSpan-p+k+1, :, etaSpan-q+1+1);
%     end
% end
% 
% Sw(1:d) = 0;
% for l = 0:q
%     Sw(1:d) = Sw(1:d) + Neta(l+1).*temp(l+1, 1:d);
% end
% Sw
% S = Sw(1:d-1)./Sw(d);