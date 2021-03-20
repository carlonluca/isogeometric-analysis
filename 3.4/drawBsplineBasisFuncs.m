%
% Project: Approximation and Finite Elements in Isogeometric Problems
% Author:  Luca Carlon
% Date:    2009.11.20
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

Xi0 = [0, 0, 0, 1, 2, 3, 4, 4, 5, 5, 5]

% Draw the basis functions.
for p = 0:5
    subplot(3, 2, p+1);
    hold on;
    box on;
    grid on;
    axis([0, 5, 0, 1]);
    Xi(1:p) = Xi0(1);
    Xi(p+1:p+1+length(Xi0)-1) = Xi0;
    Xi(p+1+length(Xi0):p+1+length(Xi0)+p) = Xi0(end);
    n = length(Xi) - p - 3;
    for i = 0:n
        xi = 0:0.01:5;
        for j = 1:length(xi)
            y(j) = computeBsplineBasis(p, n+p+1, Xi, i, xi(j));
        end
        
        for j = 2:(length(xi) - 1)
            if y(1, j) ~= 0
                if y(1, j+1) ~= 0
                    plot([xi(1, j), xi(1, j+1)], [y(1, j), y(1, j+1)],...
                        'Color', hsv2rgb([i/(n+1), 1, 1]));
                end
            end
        end
        
        clear y;
        clear x;
        [s, err] = sprintf('N_{i,%d}, i=0,...,%d', p, n);
        ylabel(s);
        xlabel('\xi');
    end
    hold off;
end