# triangle
Simple rust library for triangles.
This library implements a struct `Triangle` that has three sides of type u64.

The `pub fn build` function takes three `u64`-integers and checks if **none of those are 0** and that **the sum of two sides cannot be less than the third** to assure it is a valid triangle. If these conditions are met a struct `Triangle` is returned with the implementations for the following functions:

* `pub fn is_equilateral` checks if all sides are the same length
* `pub fn is_isosceles` checks if exactly two sides are
* `pub fn is_scalene` checks if all sides are different from each other

