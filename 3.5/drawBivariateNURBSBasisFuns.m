%
% Project: Approximation and Finite Elements in Isogeometric Problems
% Author:  Luca Carlon
% Date:    2021.03.26
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

function [xi] = drawBivariateNURBSBasisFuns(Xi, n, p, Eta, m, q, wxi, weta)
xi = 0:0.05:1;
eta = 0:0.05:1;

% Draw the basis functions.
for i = 0:n
    for j = 0:m
        subplot(n+1, m+1, (j+1)+i*(m+1));
        hold on;
        box on;
        grid on;
        colormap('jet')
        
        for bari = 1:length(xi)
            for barj = 1:length(eta)
                zeta(bari, barj) =...
                    computeNURBSBasisFun(i, xi(bari), n, p, Xi, wxi).*...
                    computeNURBSBasisFun(j, eta(barj), m, q, Eta, weta);
            end
        end
        
        surfc(xi, eta, zeta);
        shading interp;
        
        clear zeta;
        clear x;
        [s, err] = sprintf('R_{%d,%d}^{%d,%d}', i, j, p, q);
        xlabel('\xi');
        ylabel('\eta');
        zlabel(s);
        view([-102, 22]);
    end
    hold off;
end