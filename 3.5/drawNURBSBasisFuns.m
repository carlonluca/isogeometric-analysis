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

function [] = drawNURBSBasisFuns(n, p, Xi, w)
hold on;
xispace = linspace(Xi(1), Xi(length(Xi)), 1000);
R(length(xispace)) = 0;
for i = 0:n
    for j = 0:length(xispace)-1
        R(j+1) = computeNURBSBasisFun(i, xispace(j+1), n, p, Xi, w);
    end
    %plot(xispace, R, 'Color', hsv2rgb([i/(n+1), 1, 1]));
    for j = 2:(length(xispace) - 2)
        if R(j) ~= 0
            if R(j+1) ~= 0
                plot([xispace(j), xispace(j+1)], [R(j), R(j+1)],...
                    'Color', hsv2rgb([i/(n+1), 1, 1]));
            end
        end
    end
    clear R;
end
