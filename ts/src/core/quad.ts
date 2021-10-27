/**
 * Project: Approximation and Finite Elements in Isogeometric Problems
 * Author:  Luca Carlon
 * Date:    2021.10.27
 *
 * Copyright (c) 2021 Luca Carlon. All rights reserved.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

/**
 * Implements quadrature using the composite Simpson's sule.
 * 
 * @param f 
 * @param a 
 * @param b 
 * @param n 
 * @returns 
 */
export function quadSimpson(f: (x: number) => number, a: number, b: number, n: number) {
    if (a == b)
        return 0
    if (a > b)
        return -1*quadSimpson(f, b, a, n)
    
    let h = (b - a)/n
    let sum1 = 0
    let sum2 = 0
    for (let j = 1; j <= n/2 - 1; j++)
        sum1 += f(a + 2*j*h)
    for (let j = 1; j <= n/2; j++)
        sum2 += f(a + (2*j - 1)*h)

    return h/3*(f(a) + 2*sum1 + 4*sum2 + f(b))
}
