%
% Project: Approximation and Finite Elements in Isogeometric Problems
% Author:  Luca Carlon
% Date:    2009.10.30
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

% computeLagrangePoly determines the Lagrange interpolating polynomial
% interpolating the points provided.
% Input:
%   X: x coordinates of the points to be interpolated;
%   Y: y coordinates of the points to be interpolated;
%   XX: optional argument to get the values of the polynomial in the
%       values contained in the vector XX.
% Output:
%   P: coefficients of the polynomial in highest order first;
%   R: x coordinates of the N-1 extrema of the resulting polynomial;
%   S: y coordinates of the extrema.
function [P, R, S] = computeLagrangePoly(X, Y, XX)
% Make sure that X and Y are row vectors
if size(X,1) > 1;  X = X'; end
if size(Y,1) > 1;  Y = Y'; end
if size(X,1) > 1 || size(Y,1) > 1 || size(X,2) ~= size(Y,2)
  error('both inputs must be equal-length vectors')
end
N = length(X);
pvals = zeros(N,N);
% Calculate the polynomial weights for each order
for i = 1:N
  % the polynomial whose roots are all the values of X except this one
  pp = poly(X( (1:N) ~= i));
  % scale so its value is exactly 1 at this X point (and zero
  % at others, of course)
  pvals(i,:) = pp ./ polyval(pp, X(i));
end
% Each row gives the polynomial that is 1 at the corresponding X 
% point and zero everywhere else, so weighting each row by the 
% desired row and summing (in this case the polycoeffs) gives 
% the final polynomial
P = Y*pvals;
if nargin==3
  % output is YY corresponding to input XX
  YY = polyval(P,XX);
  % assign to output
  P = YY;
end
if nargout > 1
  % Extra return arguments are values where dy/dx is zero
  % Solve for x s.t. dy/dx is zero i.e. roots of derivative polynomial
  % derivative of polynomial P scales each power by its power, downshifts
  R = roots( ((N-1):-1:1) .* P(1:(N-1)) );
  if nargout > 2
    % Calculate the actual values at the points of zero derivative.
    S = polyval(P,R);
  end
end