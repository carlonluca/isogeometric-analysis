% 2DIA solves a 2D problem using Isogeometric Analysis.
% Input:
%   n: defined accordingly to the knot vector X;
%   p: degree of the NURBS surface defined in direction x;
%   X: knot vector in direction x;
%   m: defined accordingly to the knot vector Y;
%   q: degree of the NURBS surface defined in direction y;
%   Y: knot vector in direction y;
%   Pw: weighted control points.
% Output:
%function [S] = IA2D(s, f, a_1)
clear all;
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
%[n, p, Xi, m, q, Eta, P] = defineSquareParametric();
[n, p, Xi, m, q, Eta, P] = defineBsplinePlateHole();
%[n, p, Xi, m, q, Eta, P] = defineToroidalSurface();
%[n, p, Xi, m, q, Eta, P] = defineExperimental();

% Definition of the PDE.
% ======================
f = inline('1', 'x', 'y');
a_1 = inline('1', 'x', 'y');
g_N = inline('9.*x', 'x', 'y');

% Mesh refinement.
% ================
for i = 0.25:0.25:0.75
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
C(:, 1) = 2;
C(1, :) = 1;
C(n+1, :) = 1;
C(:, m+1) = 1;
% Setting the values matrix.
D = zeros(n+1, m+1);
% Setting nodes with Dirichlet boundary conditions applied.
D(:, 1) = 0;
D(1, :) = 0;
D(n+1, :) = 1;
D(:, m+1) = 2;
% Setting the edges with Neumann boundary conditions in the parametric
% domain.
N = [1, 0, 0, 0];

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
        % ki and li are the indices of the first B-spline function.
        % If the node is a Dirichlet node, then skip to the next
        % iteration.
        if C(ki+1, li+1) == 1, continue; end;
        % Integration of the homogeneous part of the force vector.
        F(a+1) = forceIntegralBsplines02...
            (n, p, Xi, m, q, Eta, P, ki, li, f, g_N, a_1, wp_force, N);
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
                            stiffnessIntegralBsplines02...
                            (n, p, Xi, m, q, Eta, P, ki, li, kj, lj,...
                            wp_stiffness, a_1);
                    end
                    continue;
                end
                % Skip the terms which can be computed by exploiting
                % symmetry.
                if b < a, b = b+1; continue; end
                ki, kj, li, lj
                % Stiffness term.
                S(a+1, b+1) = stiffnessIntegralBsplines02...
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
drawBsplineSurf(n, p, Xi, m, q, Eta, P);
%axis([-4, 0, 0, 4]);
%view([-26, 18]);
shading interp;
colorbar;