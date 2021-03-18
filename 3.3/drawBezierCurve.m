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

% Draws a Bezier curve in the 2d or 3d space from its control points. The function also
% draws the basis functions below.

function b = drawBezierCurve(P)
    % Determine the dimension.
    d = length(P(1, :));

    % Create the subplot.
    subplot(2, 1, 1);

    % Draw the Bezier line.
    u = 0:0.001:1;
    [x, y, z] = computeBezier(P, u);
    if d == 2
        plot(x, y, 'k', 'LineWidth', 2);
    else
        plot3(x, y, z, 'k', 'LineWidth', 2);
    end

    % Blocks the plot.
    hold on;

    % Draw the lines.
    for i = 1:1:(length(P(:, 1)) - 1)
        if d == 2
            plot([P(i, 1), P(i + 1, 1)], [P(i, 2), P(i + 1, 2)], 'Color', [1, 0, 0], 'LineStyle', '--');
        else
            plot3(P(i:(i + 1), 1), P(i:(i + 1), 2), P(i:(i + 1), 3), 'Color', [1, 0, 0], 'LineStyle', '--');
        end
    end

    % Draw points.
    for i = 1:1:length(P(:, 1))
        if d == 2
            plot(P(i, 1), P(i, 2), 'o', 'Color', 'k');
        else
            plot3(P(i, 1), P(i, 2), P(i, 3), 'o', 'Color', 'k');
        end
    end

    grid on;
    box on;
    title('(a)');
    xlabel('x');
    ylabel('y');

    % Add the plot with the Bernstein polynomials.
    subplot(2, 1, 2);
    hold on;

    B = inline('factorial(n)./(factorial(i).*factorial(n - i)).*u.^i.*(1 - u).^(n-i)', 'i', 'n', 'u');

    for i = 0:1:(length(P(:, 1)) - 1)
        %if mod(i + 1, 3) == 0
        %    col = [1, 0, 0];
        %elseif mod(i, 3) == 1
        %    col = [0, 1, 0];
        %else
        %    col = [0, 0, 1];
        %end
        plot(u, B(i, length(P(:, 1)) - 1, u), 'Color', hsv2rgb([i/length(P(:, 1)), 1, 1]));
    end

    % Unblocks the plot.
    hold off;
    grid on;
    box on;
    title('(b)');
    xlabel('\xi');
    ylabel('\eta');
endfunction