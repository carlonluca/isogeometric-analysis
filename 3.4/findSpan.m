% findSpan finds in which knot span a specific
% value of xi is to be found.
% Input:
%   n: max index of the control points (n+1 control points);
%   p: degree of the B-spline basis functions;
%   xi: scalar value to be found;
%   Xi: open knot vector {xi_0,...,xi_n,...,x_{n+p+1}}.
% Output:
%   i: knot span [xi_i, xi_{i+1}) in which u lies.
function i = findSpan(n, p, xi, Xi)
    % Special case.
    if xi == Xi(n+2), i = n; return; end;
    % Binary search.
    low = p;
    high = n+1;
    i = floor((low + high)./2);
    while xi < Xi(i+1) || xi >= Xi(i+2)
        if xi < Xi(i+1); high = i;
        else low = i; end;
        i = floor((low + high)./2);
    end
endfunction