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

function F = computeForceIntegralBsplines(n, p, Xi, m, q, Eta, P, ki, li, f, pw)

% Determination of the integral using the method suggested by
% prof. Hughes.
% ===========================================================
% ia = 0;
% ib = 0;
% for i = 1:2:length(pw)-1
%     % Determination of the terms of the summation.
%     ia = ia+pw(i+1).*basisFun(p, n+p+1, Xi, ki, pw(i));
%     ib = ib+pw(i+1).*basisFun(q, m+q+1, Eta, li, pw(i));
% end
% F = f.*ia.*ib;

% Determination of the integral with the 2D method.
% =================================================
% F = 0;
% for i = 1:4:length(pw)-3
%     F = F+pw(i+2)*pw(i+3)*computeForceIntegrandBsplines...
%         (n, p, Xi, m, q, Eta, P, pw(i), pw(i+1), ki, li, f);
% end

% Determination of the integral with the dblquad method.
% ======================================================
F = 0;
for xi_i = ki+1:ki+p+1
    for eta_i = li+1:li+q+1
        F = F+...
            dblquad(@(x, y)(computeForceIntegrandBsplines...
            (n, p, Xi, m, q, Eta, P, x, y, ki, li, f)),...
            Xi(xi_i), Xi(xi_i+1),...
            Eta(eta_i), Eta(eta_i+1), 1e-5);
    end
end

% Definition of the integrand function.
% =====================================
function F = computeForceIntegrandBsplines(n, p, Xi, m, q, Eta, P, xi, eta, ki, li, f)
% Preallocation.
F(length(xi)) = 0;
for l = 1:length(xi)
    % Determination of the derivatives.
    [SKL, eXi, eEta, spanxi, spaneta] =...
        computeBsplineSurfDerivsPoint(n, p, Xi, m, q, Eta, P, xi(l), eta, 1);
    % Definition of the jacobian matrix.
    DxDxi = [SKL(2, 1, 1), SKL(1, 2, 1); SKL(2, 1, 2), SKL(1, 2, 2)];
    % Evaluate the Jacobian.
    Jx = det(DxDxi);
    dki = ki-spanxi+p+1;
    dli = li-spaneta+q+1;
    if dki>0 && dki<=p+1 && dli>0 && dli<=q+1
        Nip = eXi(1, dki).*eEta(1, dli);
    else Nip = 0; end;
    % Integrand.
    F(l) = Jx.*f(SKL(1, 1, 1), SKL(1, 1, 2)).*Nip;
end