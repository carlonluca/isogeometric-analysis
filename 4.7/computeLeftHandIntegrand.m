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

function [s] = leftHandIntegrand(n, p, Xi, m, q, Eta, Pw, xi, eta, ki, li, kj, lj)
for l = 1:length(xi)
    % Determination of the derivatives.
    SKL = computeNURBSSurfDerivsPoint(n, p, Xi, m, q, Eta, Pw, xi(l), eta, 1);

    % Definition of the jacobian matrix.
    DxDxi = [SKL(1+1, 0+1, 1), SKL(1+1, 0+1, 2);...
        SKL(0+1, 1+1, 1), SKL(0+1, 1+1, 2)];

    % Evaluate the Jacobian.
    Jx = det(DxDxi);

    % Compute the inverse of the jacobian matrix.
    DxiDx = inv(DxDxi);

    % Definition of the weights.
    wixi = Pw(:, li+1, end)';
    wieta = Pw(ki+1, :, end);
    
    wjxi = Pw(:, lj+1, end)';
    wjeta = Pw(kj+1, :, end);

    % Computation of the derivatives of the NURBS bivariate basis functions.
    dRidxi = computeNURBSBasisDeriv(ki, xi(l), p, Xi, wixi).*...
        computeNURBSBasisFun(li, eta, m, q, Eta, wieta);
    dRideta = computeNURBSBasisDeriv(li, eta, q, Eta, wieta).*...
        computeNURBSBasisFun(ki, xi(l), n, p, Xi, wixi);

    dRjdxi = computeNURBSBasisDeriv(kj, xi(l), p, Xi, wjxi).*...
        computeNURBSBasisFun(lj, eta, m, q, Eta, wjeta);
    dRjdeta = computeNURBSBasisDeriv(lj, eta, q, Eta, wjeta).*...
        computeNURBSBasisFun(kj, xi(l), n, p, Xi, wjxi);

    gradRi = [dRidxi; dRideta];
    gradRj = [dRjdxi; dRjdeta];

    % Integrand.
    s(l) = Jx.*1.*(DxiDx'*gradRi)'*(DxiDx'*gradRj);
    %s(l) = gradRi'*gradRj;
end