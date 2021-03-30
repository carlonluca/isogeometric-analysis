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
alfa = inline('-1./50');
sigma = inline('0');
f = inline('x');
xi_a = 0;
xi_b = 1;
Xi = [0, 0, 1, 1];
p = 1;
n = 1;
P(1, :) = [0, 0];
P(2, :) = [1, 0];
k = findSpan(n, p, 0.5, Xi);
[n, Xi, P] = computeKnotInsertion(n, p, Xi, P, 0.5, k, 0, 1);
[S, F, u] = computeIGA1DBsplines(alfa, sigma, f, xi_a, xi_b, Xi, p);

% Plot the exact curve and the approximation.
z = Xi(1):0.001:Xi(end);
uexact = inline('x./3.*(25.*x.^2-22)');
plot(z, uexact(z), 'Color', 'red', 'LineStyle', '--');
hold on;
box on;
grid on;
xlabel('x');
ylabel('u_n');
title('(a)');

P(1, end) = xi_a;
P(2:end-1, end) = u;
P(end, end) = xi_b;

axis([0, 1, -3, 1.2]);

drawBsplineCurve(n, p, Xi, P, true, false);

% Legend.
legend('Exact', 'Approx');

% % Plot basis functions.
% subplot(2, 1, 2);
% box on;
% grid on;
% hold on;
% xlabel('x');
% ylabel('R_{i,2}');
% title('(b)');
% for i = 0:(m-p-1)
%     h = U(1):0.001:U(end);
%     plot(h(1:(length(h)-1)), Nbf(U, i, p, h(1:(length(h)-1))),...
%         'Color', hsv2rgb([i/(m-p), 1, 1]));
% end