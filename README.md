## Overview
This Rust program calculates the Lagrange interpolation polynomial from a set of given points. Lagrange interpolation is a method of constructing a polynomial that passes through a set of known points.
## Features
- **Lagrange Polynomial Calculation:** Given a set of points, the program constructs the Lagrange interpolation polynomial that passes through all those points.
- **Polynomial Operations:** The program supports polynomial addition, multiplication, and scaling.
- **Formatted Output:** The final interpolated polynomial is displayed in standard algebraic form.
## How it Works
Given a set of $n + 1$ distinct points $(x_0, y_0), (x_1, y_1), \ldots, (x_n, y_n)$, the Lagrange interpolating polynomial $P(x)$ is defined as:
	$$P(x) =\sum_{i=0}^{n} y_i L_i(x)$$
	where $L_i(x)$ are the Lagrange basis polynomials, defined as:
	$$L_i(x) = \prod\limits_{{0 \leq j \leq n}_{j \neq i}} \frac{x - x_j}{x_i - x_j}$$
