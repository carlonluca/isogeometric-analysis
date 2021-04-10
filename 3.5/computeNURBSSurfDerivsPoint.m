% computeNURBSSurfDerivPoint evaluates the derivatives of the NURBS surface S(xi, eta)
% of order up to 0<=k+l<=d, k times with respect to xi and l times with
% respect to eta.
% Input:
%   n: defined accordingly to the knot vector Xi;
%   p: degree in direction xi;
%   Xi: knot vector in direction Xi;
%   m: defined accordingl to the knot vector Eta;
%   q: degree in direction eta;
%   Eta: knot vector in direction eta;
%   Pw: weighted control points;
%   xi: value in which to evaluate the surface in the xi direction;
%   eta: value in which to evaluate the surface in the eta direction;
% Output:
%   SKL: derivatives of the NURBS surface S(xi, eta)
%        of order up to 0<=k+l<=d, k times with respect to xi and l times
%        with respect to eta. SKL(k, l) contains the derivatives of the
%        surface differentiated k times with respect to xi and l times with
%        respect to eta.
function [SKL] = computeNURBSSurfDerivsPoint(n, p, Xi, m, q, Eta, Pw, xi, eta, d)
Aders = computeBsplineSurfDerivsPoint(n, p, Xi, m, q, Eta, Pw, xi, eta, d);
wders = Aders(:, :, end);
Aders = Aders(:, :, 1:end-1);
Bders = permute(Aders, [1, 3, 2]);
SKL = zeros(d+1, d+1);
for k = 0:d
    for l = 0:d-k
        v = Bders(k+1, :, l+1);
        for j = 1:l
            v = v-nchoosek(l, j).*wders(0+1, j+1).*SKL(k+1, :, l-j+1);
        end
        for i = 1:k
            v = v-nchoosek(k, i).*wders(i+1, 0+1).*SKL(k-i+1, :, l+1);
            v2 = 0;
            for j = 1:l
                v2 = v2+nchoosek(l, j).*wders(i+1, j+1).*...
                    SKL(k-i+1, :, l-j+1);
            end
            v = v-nchoosek(k, i).*v2;
        end
        SKL(k+1, :, l+1) = v./wders(0+1, 0+1);
    end
end
SKL = permute(SKL, [1, 3, 2]);