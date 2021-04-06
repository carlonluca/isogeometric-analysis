%
% Project: Approximation and Finite Elements in Isogeometric Problems
% Author:  Luca Carlon
% Date:    2009.08.17
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

function [nBarP, barXi, mBarP, barEta, barPw] = computeSurfKnotInsertion(nP, p, Xi, mP, q, Eta, Pw, dir, knot, k, s, r)
% Rearrange control points.
Pw = permute(Pw, [1, 3, 2]);

% Case of insertion in the xi direction.
if dir == 0
    % Modify the knot vectors parameters.
    nBarP = nP+r;
    mBarP = mP;
    % Load the new knot vector in xi direction.
    barXi(1:k+1) = Xi(1:k+1);
    barXi(k+1+1:k+r+1) = knot;
    barXi(k+1+r+1:nP+p+1+r+1) = Xi(k+1+1:nP+p+1+1);
    
    % Simply copy the other direction.
    barEta = Eta(1:end);
    
    % Compute the alphas.
    for j = 1:r
        L = k-p+j;
        for i = 0:p-j-s
            alpha(i+1, j+1) = (knot-Xi(L+i+1))./(Xi(i+k+1+1)-Xi(L+i+1));
        end
    end
    
    % For each row...
    for row = 0:mP
        % Save unaltered control points.
        for i = 0:k-p, barPw(i+1, :, row+1) = Pw(i+1, :, row+1); end;
        for i = k-s:nP, barPw(i+r+1, :, row+1) = Pw(i+1, :, row+1); end;
        % Load auxiliary control points.
        for i = 0:p-s, Rw(i+1, :) = Pw(k-p+i+1, :, row+1); end;
        % Insert the knot r times.
        for j = 1:r
            L = k-p+j;
            for i = 0:p-j-s
                Rw(i+1, :) = alpha(i+1, j+1).*Rw(i+1+1, :)+(1-alpha(i+1, j+1)).*Rw(i+1, :);
            end
            barPw(L+1, :, row+1) = Rw(0+1, :);
            barPw(k+r-j-s+1, :, row+1) = Rw(p-j-s+1, :);
        end
        % Load the remaining control points.
        for i = L+1:k-s-1, barPw(i+1, :, row+1) = Rw(i-L+1, :); end;
    end
else
    % Modify the knot vectors parameters.
    nBarP = nP;
    mBarP = mP+r;
    % Load the new knot vector in xi direction.
    barEta(1:k+1) = Eta(1:k+1);
    barEta(k+1+1:k+r+1) = knot;
    barEta(k+1+r+1:mP+q+1+r+1) = Eta(k+1+1:mP+q+1+1);
    
    % Simply copy the other direction.
    barXi = Xi(1:end);
    
    % Compute the alphas.
    for j = 1:r
        L = k-q+j;
        for i = 0:q-j-s
            alpha(i+1, j+1) = (knot-Eta(L+i+1))./(Eta(i+k+1+1)-Eta(L+i+1));
        end
    end
    
    % For each row...
    for row = 0:nP
        % Save unaltered control points.
        for i = 0:k-q, barPw(row+1, :, i+1) = Pw(row+1, :, i+1); end;
        for i = k-s:mP, barPw(row+1, :, i+r+1) = Pw(row+1, :, i+1); end;
        % Load auxiliary control points.
        for i = 0:q-s, Rw(i+1, :) = Pw(row+1, :, k-q+i+1); end;
        % Insert the knot r times.
        for j = 1:r
            L = k-q+j;
            for i = 0:q-j-s
                Rw(i+1, :) = alpha(i+1, j+1).*Rw(i+1+1, :)+(1-alpha(i+1, j+1)).*Rw(i+1, :);
            end
            barPw(row+1, :, L+1) = Rw(0+1, :);
            barPw(row+1, :, k+r-j-s+1) = Rw(q-j-s+1, :);
        end
        % Load the remaining control points.
        for i = L+1:k-s-1, barPw(row+1, :, i+1) = Rw(i-L+1, :); end;
    end
end

barPw = permute(barPw, [1, 3, 2]);