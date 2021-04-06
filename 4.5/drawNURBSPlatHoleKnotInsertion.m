%
% Project: Approximation and Finite Elements in Isogeometric Problems
% Author:  Luca Carlon
% Date:    2009.10.23
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

hold on;
axis equal;
box on;
grid on;
% Definition of the control points.
P(1, 1, :) = [-1, 0, 0];
P(2, 1, :) = [-1, sqrt(2)-1, 0];
P(3, 1, :) = [1-sqrt(2), 1, 0];
P(4, 1, :) = [0, 1, 0];

P(1, 2, :) = [-2.5, 0, 0];
P(2, 2, :) = [-2.5, 0.75, 0];
P(3, 2, :) = [-0.75, 2.5, 0];
P(4, 2, :) = [0, 2.5, 0];

P(1, 3, :) = [-4, 0, 0];
P(2, 3, :) = [-4, 4, 0];
P(3, 3, :) = [-4, 4, 0];
P(4, 3, :) = [0, 4, 0];

d = length(P(1, 1, :));

% Definition of the weights.
w = ones(4, 3);
% w(2:3, 1) = w(2:3, 1).*(1+1./sqrt(2))./2;

Pw = zeros(size(P));
for k = 1:d
    Pw(:, :, k) = P(:, :, k).*w;
end
Pw(:, :, d+1) = w;

% Define the knot vectors.
Xi = [0, 0, 0, 0.5, 1, 1, 1];
Eta = [0, 0, 0, 1, 1, 1];

% Define the scalars.
n = 3;
p = 2;
m = 2;
q = 2;

subplot(2, 2, 1);
title('(a)');
drawNURBSSurf(n, p, Xi, m, q, Eta, Pw);

for i = 0.25:0.25:0.75
    k = findSpan(n, p, i, Xi);
    if Xi(k+1) ~= i
        [n, Xi, m, Eta, Pw] =...
            computeSurfKnotInsertion(n, p, Xi, m, q, Eta,...
            Pw, 0, i, k, 0, 1);
    end
    
    k = findSpan(m, q, i, Eta);
    if Eta(k+1) ~= i
        [n, Xi, m, Eta, Pw] =...
            computeSurfKnotInsertion(n, p, Xi, m, q, Eta,...
            Pw, 1, i, k, 0, 1);
    end
end

subplot(2, 2, 2);
title('(b)');
drawNURBSSurf(n, p, Xi, m, q, Eta, Pw);

for i = 0.125:0.125:0.875
    k = findSpan(n, p, i, Xi);
    if Xi(k+1) ~= i
        [n, Xi, m, Eta, Pw] =...
            computeSurfKnotInsertion(n, p, Xi, m, q, Eta,...
            Pw, 0, i, k, 0, 1);
    end
    
    k = findSpan(m, q, i, Eta);
    if Eta(k+1) ~= i
        [n, Xi, m, Eta, Pw] =...
            computeSurfKnotInsertion(n, p, Xi, m, q, Eta,...
            Pw, 1, i, k, 0, 1);
    end
end

subplot(2, 2, 3);
title('(c)');
drawNURBSSurf(n, p, Xi, m, q, Eta, Pw);

for i = 0.0625:0.0625:0.9375
    k = findSpan(n, p, i, Xi);
    if Xi(k+1) ~= i
        [n, Xi, m, Eta, Pw] =...
            computeSurfKnotInsertion(n, p, Xi, m, q, Eta,...
            Pw, 0, i, k, 0, 1);
    end
    
    k = findSpan(m, q, i, Eta);
    if Eta(k+1) ~= i
        [n, Xi, m, Eta, Pw] =...
            computeSurfKnotInsertion(n, p, Xi, m, q, Eta,...
            Pw, 1, i, k, 0, 1);
    end
end

subplot(2, 2, 4);
title('(d)');
drawNURBSSurf(n, p, Xi, m, q, Eta, Pw);


% % Draw the parametric domain.
% % ===========================
% subplot(1, 2, 2);
% hold on;
% axis equal;
% box on;
% grid on;
% 
% % Draw the domain.
% X = [0, 1];
% Y = [0, 1];
% Z = [0, 0; 0, 0];
% surfc(X, Y, Z);
% 
% parXi = [0, 0, 0, 0, 1, 1, 1, 1];
% parEta = [0, 0, 0, 1, 1, 1];
% parn = 3;
% parp = 3;
% parm = 2;
% parq = 2;
% 
% % Draw the isocurves on Eta.
% subplot(1, 2, 1);
% for j = 1:length(Eta)
%     for i = 1:length(spacexi)
%         S(i, :) = NURBSSurfPoint(n, p, Xi, m, q, Eta, Pw, spacexi(i), Eta(j));
%     end
%     plot3(S(:, 1), S(:, 2), S(:, 3));
% end
% 
% % Draw the isocurves on Xi.
% for i = 1:length(Xi)
%     for j = 1:length(spaceeta)
%         S(j, :) = NURBSSurfPoint(n, p, Xi, m, q, Eta, Pw, Xi(i), spaceeta(j));
%     end
%     plot3(S(:, 1), S(:, 2), S(:, 3));
% end
% 
% % Draw the isocurves in the parametric domain.
% discXi = [0.5];
% discEta = [];
% for i = 1:length(discXi)
%     subplot(1, 2, 2);
%     line([discXi(i), discXi(i)], [0, 1], [0, 0]);
%     subplot(1, 2, 1);
%     for j = 1:length(spaceeta)
%         parS(j, :) = NURBSSurfPoint(parn, parp, parXi, parm, parq, parEta, Pw, discXi(i), spaceeta(j));
%     end
%     plot3(parS(:, 1), parS(:, 2), parS(:, 3));
%     clear parS;
% end
% for j = 1:length(discEta)
%     subplot(1, 2, 2);
%     line([0, 1], [discEta(j), discEta(j)], [0, 0]);
%     subplot(1, 2, 1);
%     for i = 1:length(spacexi)
%         parS(i, :) = NURBSSurfPoint(parn, parp, parXi, parm, parq, parEta, Pw, spacexi(i), discEta(j));
%     end
%     plot3(parS(:, 1), parS(:, 2), parS(:, 3));
%     clear parS;
% end