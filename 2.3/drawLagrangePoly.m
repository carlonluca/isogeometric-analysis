%
% Project: Approximation and Finite Elements in Isogeometric Problems
% Author:  Luca Carlon
% Date:    2009.10.30
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

X = [1 2 3 4 5 6 7 9 9.5];
Y = [0 0 0 2 1 1 1 4 -1];

xx = 0.5 : 0.01 : 14;

hold on;

for i = 5:9
    [P, R, S] = computeLagrangePoly(X(1:i), Y(1:i));
    plot(xx, polyval(P, xx), 'Color', hsv2rgb([(i-5)/5, 1, 1]));
end

plot(X, Y, '.', 'Color', 'black');

grid;
axis([0.5 10 -13 20]);
box on;
xlabel('x');
ylabel('y');