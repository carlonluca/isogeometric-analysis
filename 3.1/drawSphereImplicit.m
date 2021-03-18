% his program is free software: you can redistribute it and/or modify
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
% along with this program.  If not, see <https://www.gnu.org/licenses/>.
%
% Copyright © 2008-2021 Luca Carlon

%
% Author: Luca Carlon
%

% Draws a sphere using an implicit equation.

[x, y, z] = ndgrid(-1:0.1:1, -1:0.1:1, -1:0.1:1);
F = x.^2 + y.^2 + z.^2 - 1;
isosurface(F, 0);

% Export.
% exportfig(gcf, 'sphere.eps');