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

% Compute B-spline surface derivatives in the specified point
% from order 0 to order d.
% Input:
%   n: defined accordingly to the knot vector Xi;
%   p: degree in direction xi;
%   Xi: knot vector in direction xi;
%   m: defined accordingly to the knot vector Eta;
%   q: degree in direction eta;
%   Eta: not vector in direction eta;
%   P: control points written in the format P(xi, eta, x_k);
%   xi: point in direction xi in which the surface is to be determined;
%   eta: point in dicrection eta in which the surface is to be determined;
%   d: derivatives are to be computed up to order d.
% Output:
%   SKL: tridimensional matrix where SKL_{k,l,i} is the ith
%        coordinate derivative of S(xi, eta) with respect to
%        xi k times and to eta l times.
%   Nxi: returns derivsBasisFuns(xiSpan, xi, p, d, Xi);
%   Neta: returns derivsBasisFuns(etaSpan, eta, q, d, Eta);
%   spanxi: returns spanxi = findSpan(n, p, xi, Xi);
%   spaneta: returns findSpan(m, q, eta, Eta).
function [SKL, Nxi, Neta, spanxi, spaneta] = computeBsplineSurfDerivPoint(n, p, Xi, m, q, Eta, P, xi, eta, d)
   % Determine the span in which the point [xi, eta]^T is.
   spanxi = findSpan(n, p, xi, Xi);
   spaneta = findSpan(m, q, eta, Eta);
   % Determine the derivatives.
   Nxi = computeBsplineBasisDeriv(spanxi, xi, p, d, Xi);
   Neta = computeBsplineBasisDeriv(spaneta, eta, q, d, Eta);
   SKL = zeros(d+1, length(P(1, 1, :)), d+1);
   for k = 0:d
      for l = 0:d
         for i = 0:length(P(1, 1, :))-1
               SKL(k+1, i+1, l+1) = Neta(l+1, :)*P(spanxi-p+1:spanxi+1,...
                  spaneta-q+1:spaneta+1, i+1)'*Nxi(k+1, :)';
         end
      end
   end
   SKL = permute(SKL, [1, 3, 2]);

   % Piegl version.
   % ==============
   % for k=0:dxi
   %     for s=0:q
   %         temp = zeros(s+1, length(P(1, 1, :)));
   %         for r=0:q
   %             temp(s+1, :) = temp(s+1, :)+...
   %                 Nxi(k+1, r+1).*Q(xiSpan-p+r+1, :, etaSpan-q+s+1);
   %         end
   %     end
   %     dd = min([d-k, deta]);
   %     for l=0:dd
   %         for s=0:q
   %             SKL(k+1, :, l+1) = SKL(k+1, :, l+1)+...
   %                 Neta(l+1, s+1).*temp(s+1, :);
   %         end
   %     end
   % end
   % SKL = permute(SKL, [1, 3, 2]);
endfunction