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
## Example
- Given the points $(0,4), (−2,1)$, and $(2,3)$, the program computes the interpolated polynomial.
## Example Output
>```
>The interpolated polynomial is: 0.33x^2 + 0.83x + 4.00

## Requirements
- Rust installed on your machine. (If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it).

## Contributing
  - If you intend to contribute to this project, fork the repository and make a pull request.

## Usage
- To use this code, you can clone or download this repository.
- Compile and run the Rust program using the following command:
>```
>cargo build
>cargo run
## Acknowledgments
- Rust

## Clone the repository:

   ```bash
   git clone https://github.com/cypriansakwa/Lagrange_Interpolation_Polynomial_from_Given_Points_in_Rust.git
   cd Lagrange_Interpolation_Polynomial_from_Given_Points_in_Rust
