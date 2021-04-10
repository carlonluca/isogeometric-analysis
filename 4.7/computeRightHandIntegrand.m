%
% Project: Approximation and Finite Elements in Isogeometric Problems
% Author:  Luca Carlon
% Date:    2009.11.24
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

function [F] = rightHandIntegrand(n, p, Xi, m, q, Eta, Pw, xi, eta, ki, li, f)
for l = 1:length(xi)
    % Determination of the derivatives.
    SKL = computeNURBSSurfDerivsPoint(n, p, Xi, m, q, Eta, Pw, xi(l), eta, 1);

    % Definition of the jacobian matrix.
    DxDxi = [SKL(1+1, 0+1, 1), SKL(1+1, 0+1, 2);...
        SKL(0+1, 1+1, 1), SKL(0+1, 1+1, 2)];

    % Evaluate the Jacobian.
    Jx = det(DxDxi);

    % Definition of the weights.
    wixi = Pw(:, li+1, end)';
    wieta = Pw(ki+1, :, end);
    
    Rip = computeNURBSBasisFun(ki, xi(l), n, p, Xi, wixi).*computeNURBSBasisFun(li, eta, m, q, Eta, wieta);

    % Integrand.
    F(l) = Jx.*(1.*f).*(Rip);
    %F(l) = f.*Rip;
end