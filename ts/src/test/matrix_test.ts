import { Matrix2 } from "../core/matrix";

let m1 = new Matrix2([[1, 2, 3]])
m1.print()

let m2 = new Matrix2([[1, 1, 1]])
m2.print()

let m3 = Matrix2.add(m1, m2)
m3.print()