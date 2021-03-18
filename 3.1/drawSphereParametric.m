%
% Project: Approximation and Finite Elements in Isogeometric Problems
% Author:  Luca Carlon
% Date:    -
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

% Draws a sphere using a parametric equation.

XIV = linspace(0, pi, 100);
NUV = linspace(0, 2*pi, 100);
[XI, NU] = meshgrid(XIV, NUV);
X = sin(XI).*cos(NU);
Y = sin(XI).*sin(NU);
Z = cos(XI);
mesh(X, Y, Z);
axis equal;
box on;
colorbar;
grid on;