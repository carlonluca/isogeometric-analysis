function s = computeStiffnessIntegralBsplines02(n, p, Xi, m, q, Eta, P, ki, li, kj, lj, pw, a_1)

% Determination of the two-dimensional method.
% ============================================
% s = 0;
% % Iteration over all the couples of integration nodes.
% for i = 1:4:length(pw)-3
%     s = s+pw(i+2)*pw(i+3)*computeStiffnessIntegrandBsplines...
%             (n, p, Xi, m, q, Eta, P, pw(i), pw(i+1),...
%             ki, li, kj, lj, a_1);
% end

% Determination of the integral with the dblquad method.
% ======================================================
s = 0;
% According to the properties of the B-spline basis functions, each
% function is nonzero only in a specified interval. This property is useful
% in order to reduce the domain of computation of the integral.
a = max([(ki+1), (kj+1)]);
b = min([(ki+p+1+1), (kj+p+1+1)])-1;
c = max([(li+1), (lj+1)]);
d = min([(li+q+1+1), (lj+q+1+1)])-1;
% The integral is divided over the single elements as along the edges the
% degree of continuity is not known: it could lead to a wrong computation
% of the integral.
for xi_i = a:b
    for eta_i = c:d
        s = s+dblquad(@(x, y)computeStiffnessIntegrandBsplines...
            (n, p, Xi, m, q, Eta, P, x, y,...
            ki, li, kj, lj, a_1), Xi(xi_i), Xi(xi_i+1),...
            Eta(eta_i), Eta(eta_i+1));
    end
end

function s = computeStiffnessIntegrandBsplines(n, p, Xi, m, q, Eta, P, xi, eta, ki, li, kj, lj, a_1)
% Preallocation.
s(length(xi)) = 0;
oldeta = eta;
% Iteration on all the requested values on which to eval the integrand.
for l = 1:length(xi)
    moveForward = false;
    moveXi = false;
    moveMax = 100;
    moveCurrent = 0;
    eta = oldeta;
    Jx = 0;
    rcond = 1e-5;
    % I want to avoid the determinant of the Jacobian matrix to vanish.
    % This happens when a singularity is found and it is not possible
    % to translate the function to the parametric domain. In this case
    % I simply move by a small percentage the point in which the
    % integrand is evaluated.
    while Jx == 0
        % Determination of the derivatives. This calculation involves
        % the calculation of the value of the bsis functions in the
        % points and of the knot spans in which the points are located.
        % I can re-use these values later on, avoiding new evauations.
        [SKL, eXi, eEta, spanxi, spaneta] =...
            computeBsplineSurfDerivsPoint(n, p, Xi, m, q, Eta, P, xi(l), eta, 1);
        % Definition of the jacobian matrix.
        DxDxi = [SKL(2, 1, 1), SKL(1, 2, 1); SKL(2, 1, 2), SKL(1, 2, 2)];
        % Evaluate the Jacobian.
        Jx = det(DxDxi);
        if Jx == 0, Jx, end;
        xi(l);
        eta;
        % If the Jacobian is 0, the matrix is singular and therefore not
        % invertible. This is not acceptable as I need it to be
        % invertible.
        if Jx ~= 0, break; end;
        % Check to see if we still have to move in the same direction.
        if moveXi == true
            % If possible move back on xi in the parametric space.
            if moveForward == true && xi(l) >= Xi(end)-rcond
                moveForward = false;
            end
            if moveForward == false && xi(l) <= Xi(1)+rcond
                moveForward = true;
            end
        else
            % If possible move back on eta in the parametric space.
            if moveForward == true && eta >= Eta(end)-rcond
                moveForward = false;
            end
            if moveForward == false && eta <= Eta(1)+rcond
                moveForward = true;
            end
        end
        moveCurrent = moveCurrent+1;
        if moveCurrent >= moveMax, moveXi = ~moveXi; end;
        if moveXi == true
            if moveForward == false, xi(l) = xi(l)-rcond;
            else xi(l) = xi(l)+rcond; end;
        else
            if moveForward == false, eta = eta-rcond;
            else eta = eta+rcond; end;
        end
    end
    % Compute the inverse of the jacobian matrix.
    DxiDx = inv(DxDxi);
    dki = ki-spanxi+p+1;
    dli = li-spaneta+q+1;
    dkj = kj-spanxi+p+1;
    dlj = lj-spaneta+q+1;
    % According to the local support property of the B-spline functions
    % they are zero outside their local support.
    if dki>0 && dki<=p+1 && dli>0 && dli<=q+1
        dNidxi = eXi(2, dki).*eEta(1, dli);
    else dNidxi = 0; end;
    if dli>0 && dli<=q+1 && dki>0 && dki<=p+1
        dNideta = eEta(2, dli).*eXi(1, dki);
    else dNideta = 0; end;
    if dkj>0 && dkj<=p+1 && dlj>0 && dlj<=q+1
        dNjdxi = eXi(2, dkj).*eEta(1, dlj);
    else dNjdxi = 0; end;
    if dlj>0 && dlj<=q+1 && dkj>0 && dkj<=p+1
        dNjdeta = eEta(2, dlj).*eXi(1, dkj);
    else dNjdeta = 0; end;
    % Definition of the gradients.
    gradNi = [dNidxi; dNideta];
    gradNj = [dNjdxi; dNjdeta];
    % Evaluation of the integrand.
    s(l) = Jx.*1.*(DxiDx'*gradNi)'*(DxiDx'*gradNj).*...
        a_1(SKL(1, 1, 1), SKL(1, 1, 2));
end

% Determination of the integral through the method proposed by
% prof. Hughes.
% ============================================================
% a = @(x)basisFun(q, m+q+1, Eta, li, x).*...
%     basisFun(q, m+q+1, Eta, lj, x);
% b = @(x)derivsBasisFun(p, Xi, ki, x, 1).*...
%     derivsBasisFun(p, Xi, kj, x, 1);
% c = @(x)basisFun(p, n+p+1, Xi, ki, x).*...
%     basisFun(p, n+p+1, Xi, kj, x);
% d = @(x)derivsBasisFun(q, Eta, li, x, 1).*...
%     derivsBasisFun(q, Eta, lj, x, 1);
% 
% ia = 0;
% ib = 0;
% ic = 0;
% id = 0;
% for i = 1:2:length(wxiN)-1
%     ia = ia+wxiN(i+1).*a(wxiN(i));
%     ic = ic+wxiN(i+1).*c(wxiN(i));
% end
% for i = 1:2:length(wxidN)-1
%     ib = ib+wxidN(i+1).*b(wxidN(i));
%     id = id+wxidN(i+1).*d(wxidN(i));
% end
% s = ia.*ib(2)+ic.*id(2);