%
% Project: Approximation and Finite Elements in Isogeometric Problems
% Author:  Luca Carlon
% Date:    2009.11.24
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

% Load coefficients for the integration of the stiffness terms.
load wxi_p4_q4_kXi0_kEta0_4elementsXi_4elementsEta;
%load wp_p4_q4_k5_l5_rXi0_rEta0_TolX-5_TolFun-5
wp_stiffness = wxi;
% Load coefficients for the integration of the force terms.
%load wxi_p2_k0_4elements
load wp_p2_q2_k4_l4_rXi1_rEta1;
wp_force = wxi;

% Definition of the model.
% ========================
%[n, p, Xi, m, q, Eta, P] = defineParametricSquare();
[n, p, Xi, m, q, Eta, P] = defineBsplinePlateHole();

% Definition of the PDE.
% ======================
f = inline('1', 'x', 'y');
a_1 = inline('1', 'x', 'y');

% Mesh refinement.
% ================
for i = 0.1:0.1:0.9
    k = findSpan(n, p, i, Xi);
    if Xi(k+1) ~= i
        [n, Xi, m, Eta, P] =...
            computeSurfKnotInsertion(n, p, Xi, m, q, Eta,...
            P, 0, i, k, 0, 1);
    end
    k = findSpan(m, q, i, Eta);
    if Eta(k+1) ~= i
        [n, Xi, m, Eta, P] =...
            computeSurfKnotInsertion(n, p, Xi, m, q, Eta,...
            P, 1, i, k, 0, 1);
    end
end

% Boundary conditions.
% ====================
% Connectivity matrix.
C = zeros(n+1, m+1);
% Setting nodes with Dirichlet boundary conditions applied:
%   0: no constraint is applied;
%   1: means Dirichlet condition is applied;
%   2: means Neumann condition is applied.
C(1, :) = 1;
C(n+1, :) = 1;
C(:, 1) = 1;
C(:, m+1) = 1;
% Setting the values matrix.
D = zeros(n+1, m+1);
% Setting nodes with Dirichlet boundary conditions applied.
D(1, :) = 0;
D(n+1, :) = 0;
D(:, 1) = 0;
D(:, m+1) = 1;

% Computation of the matrices.
% ============================
% Preallocation of the stiffness matrix and of the force vector.
S(sum(sum(C==0)), sum(sum(C==0))) = 0;
F(sum(sum(C==0))) = 0;
% Indices of the matrices.
a = 0;
b = 0;
for ki = 0:n
    for li = 0:m
        printf("Working: %f%%\n", (n*ki + li)/(n*m)*100)

        % ki and li are the indices of the first B-spline function.
        % If the node is a Dirichlet node, then skip to the next
        % iteration.
        if C(ki+1, li+1) == 1, continue; end;
        % Integration of the homogeneous part of the force vector.
        F(a+1) = computeForceIntegralBsplines(n, p, Xi, m, q, Eta, P, ki, li, f, wp_force);
        % Iteration on the second B-spline basis function.
        for kj = 0:n
            for lj = 0:m
                % If the node belongs to the set of Dirichlet nodes then
                % it is taken into account only for the force vector.
                if C(kj+1, lj+1) == 1
                    % If the Dirichlet condition is homogeneous then
                    % it is convenient to short-circuited so that 
                    % the integral is not computed.
                    if D(kj+1, lj+1) ~= 0
                        % The term is subtracted from the force vector
                        % term.
                        F(a+1) = F(a+1)-D(kj+1, lj+1).*...
                            computeStiffnessIntegralBsplines...
                            (n, p, Xi, m, q, Eta, P, ki, li, kj, lj,...
                            wp_stiffness, a_1);
                    end
                    continue;
                end
                % Skip the terms which can be computed by exploiting
                % symmetry.
                if b < a, b = b+1; continue; end
                % Stiffness term.
                S(a+1, b+1) = computeStiffnessIntegralBsplines...
                    (n, p, Xi, m, q, Eta, P, ki, li, kj, lj,...
                    wp_stiffness, a_1);
                b = b+1;
            end
        end
        a = a+1;
        b = 0;
    end
end

% Complete the lower triangle (symmetry).
for i = 1:length(S(1, :))
    for j = 1:i
        S(i, j) = S(j, i);
    end
end

% Solution of the linear system.
% ==============================
S
F = F'
Uv = linsolve(S, F)

% Creation of the solution function.
% ==================================
a = 0;
U = zeros(size(P(:, :, 1)));
for ki = 0:n
    for li = 0:m
        if C(ki+1, li+1) == 1, continue; end;
        U(ki+1, li+1) = Uv(a+1);
        a = a+1;
    end
end
U = U+D;
P(:, :, 3) = U;
colormap("jet");
drawBsplineSurf(n, p, Xi, m, q, Eta, P(:, :, 1:3));
axis([-4, 0, 0, 4]);
view([-26, 18]);
shading interp;
colorbar;
