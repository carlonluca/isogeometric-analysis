%
% Project: Approximation and Finite Elements in Isogeometric Problems
% Author:  Luca Carlon
% Date:    2009.04.07
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

% Example of FEM to solve the diff equation

for n = 0:2:6
    clear y;
    clear u;
    subplot(2, 2, n./2+1);
    hold on;
    
    xlabel('x');
    ylabel('y');
    grid on;
    box on;
    [t, err] = sprintf('M_h = %d', n+1);
    title(t);
    ua = 0;
    ub = 1;
    
    [S, F, u] = computeFiniteElementExample(@(x) -1, @(x) 0, @(x) 10,...
        [0, 1], ua, ub, n);
    
    x = linspace(0, 1, n+2);
    gamma = @(y) (ua.*computePhi(x(1), 1/((1-0)/(n+1)), y) +...
        ub.*computePhi(x(end), 1/((1-0)/(n+1)), y));
    
    z = 0:0.01:1;
    y = zeros(1, length(z));
    for i = 1:length(z)
        y(1, i) = 0;
        for j = 1:length(u)
            y(1, i) = y(1, i) + u(j).*computePhi(x(j), 1/((1-0)/(n+1)), z(i));
        end
        y(1, i) = y(1, i) + gamma(z(i));
    end
    
    p1 = plot(z, y, 'Color', 'k');
    hold on;
    uexact = inline('x.*(5.*x-4)');
    p2 = plot(z, uexact(z), 'Color', 'red');
    
    for i = 2:(length(u)-1)
        plot(x(i), u(i), 'o', 'Color', 'k');
    end
    plot(x(1), ua, 'o', 'Color', 'k');
    plot(x(end), ub, 'o', 'Color', 'k');

    % Legend.
    legend([p2, p1], 'Exact', 'Approx', 'Location', 'southeast');
end

hold off;
