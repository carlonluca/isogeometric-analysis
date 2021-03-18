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
% along with this program.  If not, see <https://www.gnu.org/licenses/>.
%
% Copyright © 2008-2021 Luca Carlon

%
% Author: Luca Carlon
%

% Draws an ellipse using a parametric equation.

t=linspace(0,2*3.14);
X=4*cos(t);
Y=2*sin(t);
g=plot(X,Y);
axis equal
axis([-4.5, 4.5, -2.5, 2.5]);
set(g, 'Color', 'black');
title('(a)');
grid on

% Export.
% exportfig(gcf, 'ellipse.eps');