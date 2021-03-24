%
% Project: Approximation and Finite Elements in Isogeometric Problems
% Author:  Luca Carlon
% Date:    2009.08.16
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

Xi = [0, 0, 0, 0, 1, 4, 6, 8, 8, 8, 8];
w = [1, 1, 1, 3, 1, 1, 1];
p = 3;

% Draw the basis functions.
subplot(2, 1, 1);
hold on;
box on;
grid on;
title('(a)');
axis([Xi(1), Xi(end), 0, 1]);
n = length(Xi) - p - 2;
for i = 0:n
    xi = Xi(1):0.01:Xi(end);
    for j = 1:length(xi)
        y(j) = computeNURBSBasisFun(i, xi(j), n, p, Xi, w);
    end
    
    for j = 1:length(xi)-1
        if y(1, j) ~= 0 && y(1, j+1) ~= 0
            plot([xi(1, j), xi(1, j+1)], [y(1, j), y(1, j+1)],...
                'Color', hsv2rgb([i/(n+1), 1, 1]));
        end
    end

    clear y;
    clear x;
    [s, err] = sprintf('R_{i}^{%d}, i=0,...,%d', p, n);
    ylabel(s);
    xlabel('\xi');
end

subplot(2, 1, 2);
hold on;
box on;
grid on;
title('(b)');
axis([Xi(1), Xi(end), -1.5, 1.5]);
n = length(Xi) - p - 2;
for i = 0:n
    xi = Xi(1):0.01:Xi(end);
    for j = 1:length(xi)
        y(j) = computeNURBSBasisDeriv(i, xi(j), p, Xi, w);
    end

    for j = 2:(length(xi) - 2)
        if y(j) ~= 0
            if y(j+1) ~= 0
                plot([xi(j), xi(j+1)], [y(j), y(j+1)],...
                    'Color', hsv2rgb([i/(n+1), 1, 1]));
            end
        end
    end

    clear y;
    clear x;
    [s, err] = sprintf('R_{i}^{%d}'', i=0,...,%d', p, n);
    ylabel(s);
    xlabel('\xi');
end