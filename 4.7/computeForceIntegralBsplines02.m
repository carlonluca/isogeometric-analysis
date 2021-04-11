function F = computeForceIntegralBsplines02(n, p, Xi, m, q, Eta, P, ki, li, f, g_N, a_1, pw, N)
% Determination of the integral using the method suggested by
% prof. Hughes.
% ===========================================================
% ia = 0;
% ib = 0;
% for i = 1:2:length(pw)-1
%     % Determination of the terms of the summation.
%     ia = ia+pw(i+1).*basisFun(p, n+p+1, Xi, ki, pw(i));
%     ib = ib+pw(i+1).*basisFun(q, m+q+1, Eta, li, pw(i));
% end
% F = f.*ia.*ib;

% Determination of the integral with the 2D method.
% =================================================
% F = 0;
% for i = 1:4:length(pw)-3
%     F = F+pw(i+2)*pw(i+3)*forceIntegrandBsplines...
%         (n, p, Xi, m, q, Eta, P, pw(i), pw(i+1), ki, li, f);
% end

% Determination of the integral with the dblquad method.
% ======================================================
F = 0;
% According to the properties of the B-spline basis functions, each
% function is nonzero only in a specified interval. This property is useful
% in order to reduce the domain of computation of the integral.
% The integral is divided over the single elements as along the edges the
% degree of continuity is not known: it could lead to a wrong computation
% of the integral.
for xi_i = ki+1:ki+p+1
    for eta_i = li+1:li+q+1
        F = F+...
            dblquad(@(x, y)(computeForceIntegrandBsplines...
            (n, p, Xi, m, q, Eta, P, x, y, ki, li, f)),...
            Xi(xi_i), Xi(xi_i+1),...
            Eta(eta_i), Eta(eta_i+1), 1e-5);
    end
end

% N stores the definition of which edges of the parametric domain has to be
% considered part of the Neumann boundary.
for i = 1:length(N)
    if N(i) == 0, continue; end;
    % Determine the domain of the line integral to compute.
    switch i
        case 1
            if li~=0, continue; end;
            A = [Xi(1), Eta(1)]; B = [Xi(end), Eta(1)];
            domain(1) = Xi(ki+1);
            domain(2) = Xi(ki+p+2);
        case 2
            if ki~=n, continue; end;
            A = [Xi(end), Eta(1)]; B = [Xi(end), Eta(end)];
            domain(1) = Eta(li+1);
            domain(2) = Eta(li+q+2);
        case 3
            if li~=m, continue; end;
            A = [Xi(1), Eta(end)]; B = [Xi(end), Eta(end)];
            domain(1) = Xi(ki+1);
            domain(2) = Xi(ki+p+2);
        case 4
            if ki~=0, continue; end;
            A = [Xi(1), Eta(1)]; B = [Xi(1), Eta(end)];
            domain(1) = Eta(li+1);
            domain(2) = Eta(li+q+2);
    end
    % Computation of the integral.
    F = F+quad(@(x)(computeNeumannIntegrand(n, p, Xi, m, q, Eta,...
                P, ki, li, a_1, g_N, x, A, B)),...
                domain(1), domain(2));
end

% Definition of the integrand function.
% =====================================
function F = computeForceIntegrandBsplines...
    (n, p, Xi, m, q, Eta, P, xi, eta, ki, li, f)
% Preallocation.
F(length(xi)) = 0;
for l = 1:length(xi)
    % Determination of the derivatives.
    [SKL, eXi, eEta, spanxi, spaneta] =...
        computeBsplineSurfDerivsPoint(n, p, Xi, m, q, Eta, P, xi(l), eta, 1);
    % Definition of the jacobian matrix.
    DxDxi = [SKL(2, 1, 1), SKL(1, 2, 1); SKL(2, 1, 2), SKL(1, 2, 2)];
    % Evaluate the Jacobian.
    Jx = det(DxDxi);
    dki = ki-spanxi+p+1;
    dli = li-spaneta+q+1;
    if dki>0 && dki<=p+1 && dli>0 && dli<=q+1
        Nip = eXi(1, dki).*eEta(1, dli);
    else Nip = 0; end;
    % Integrand.
    F(l) = Jx.*f(SKL(1, 1, 1), SKL(1, 1, 2)).*Nip;
end

function N = computeNeumannIntegrand(n, p, Xi, m, q, Eta, P, ki, li,...
    a_1, g_N, t, A, B)
N(length(t)) = 0;
for i = 1:length(t)
    % Definition of the map.
    chi = (1-t(i)).*A + t(i).*B;
    % Determination of the derivatives.
    [SKL, eXi, eEta, spanxi, spaneta] =...
        computeBsplineSurfDerivsPoint(n, p, Xi, m, q, Eta, P, chi(1), chi(2), 1);
    % Definition of the jacobian matrix.
    DxDxi = [SKL(2, 1, 1), SKL(1, 2, 1); SKL(2, 1, 2), SKL(1, 2, 2)];
    dki = ki-spanxi+p+1;
    dli = li-spaneta+q+1;
    if dki>0 && dki<=p+1 && dli>0 && dli<=q+1
        Nip = eXi(1, dki).*eEta(1, dli);
    else
        Nip = 0;
    end
    N(i) = a_1(SKL(1, 1, 1), SKL(1, 1, 2)).*...
        g_N(SKL(1, 1, 1), SKL(1, 1, 2)).*...
        Nip.*norm(DxDxi*(-A+B)'./norm(-A+B));
end