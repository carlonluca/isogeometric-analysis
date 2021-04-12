%
% Project: Approximation and Finite Elements in Isogeometric Problems
% Author:  Luca Carlon
% Date:    2021.04.12
%
% Copyright (c) 2021 Luca Carlon. All rights reserved.
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

[n, p, Xi, m, q, Eta, P, d, w] = defineNURBSToroid();

hold on;
axis([-6, 6, -6, 6, -2, 2]);
colormap("jet");
colorbar;
axis equal;
box on;
grid on;
xlabel('x');
ylabel('y');
zlabel('z');

Pw = zeros(size(P));
for k = 1:d
    Pw(:, :, k) = P(:, :, k).*w;
end
Pw(:, :, d+1) = w;

drawNURBSSurf(n, p, Xi, m, q, Eta, Pw);