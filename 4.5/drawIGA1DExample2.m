%
% Project: Approximation and Finite Elements in Isogeometric Problems
% Author:  Luca Carlon
% Date:    2009.11.06
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

clear all;
alfa = @(x) -1./50;
sigma = @(x) 0;
f = @(x) x;
xi_a = 0;
xi_b = 1;
p = 1;

for s = 1:1:6
    clear y;
    clear P;
    subplot(3, 2, s);

    P(1, :) = [0, 0];
    P(2, :) = [1, 0];
    Xi = [0, 0, 1, 1];
    knots = linspace(0, 1, s + 2);
    n = length(Xi) - p - 2;
    for j = 2:1:(size(knots)(2) - 1)
        k = findSpan(n, p, knots(j), Xi);
        [n, Xi, P] = computeKnotInsertion(n, p, Xi, P, knots(j), k, 0, 1);
    end

    [S, F, u] = computeIGA1DBsplines(alfa, sigma, f, xi_a, xi_b, Xi, p);

    % Plot the exact curve and the approximation.
    z = Xi(1):0.001:Xi(end);
    uexact = @(x) x./3.*(25.*x.^2-22);
    plot(z, uexact(z), 'Color', 'red', 'LineStyle', '--');
    hold on;
    box on;
    grid on;
    xlabel('x');
    ylabel('u_n');
    title(strcat('\Xi = ', mat2str(Xi, 2)), "fontsize", 6);

    P(1, end) = xi_a;
    P(2:end-1, end) = u;
    P(end, end) = xi_b;

    axis([0, 1, -3, 1.2]);

    drawBsplineCurve(n, p, Xi, P, true, false);

    % Legend.
    %legend('Exact', 'Approx');
end
