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

% Draws a sphere using an implicit equation.

[x, y, z] = ndgrid(-1:0.1:1, -1:0.1:1, -1:0.1:1);
F = x.^2 + y.^2 + z.^2 - 1;
iso = .0;
colormap("jet");
colorbar;
axis equal;
[faces, verts, colors] = isosurface(x, y, z, F, iso, z);
patch('Vertices', verts, 'Faces', faces, ...
      'FaceVertexCData', colors, ...
      'FaceColor', 'interp', ...
      'edgecolor', 'interp');
xlabel("x");
ylabel("y");
zlabel("z");

% Export.
% exportfig(gcf, 'sphere.eps');