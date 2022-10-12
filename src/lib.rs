pub struct Triangle {
    a: u64,
    b: u64,
    c: u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let [a, b, c] = sides;
        // Test if either side is 0 and if the sum of two sides is less than the third
        if a == 0 || b == 0 || c == 0 || a + b < c || a + c < b || b + c < a {
            return None;
        }
        // Return some Triangle if no side is 0 and it passes the inequality check
        Some(Triangle { a, b, c })
    }

    // An equilateral triangle has three sides of the same length
    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }
    // A scalene triangle has three sides with different lengths
    pub fn is_scalene(&self) -> bool {
        !self.is_equilateral() && !self.is_isosceles()
    }
    // An isoscele triangle has two sides of the same length
    pub fn is_isosceles(&self) -> bool {
        !self.is_equilateral()
            && ((self.a == self.b && self.a != self.c)
                || (self.b == self.c && self.b != self.a)
                || (self.a == self.c && self.a != self.b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_length_sides_are_ok() {
        let sides = [2, 2, 2];

        let triangle = Triangle::build(sides);

        assert!(triangle.is_some());
    }

    #[test]
    #[ignore]
    fn zero_length_sides_are_illegal() {
        let sides = [0, 0, 0];

        let triangle = Triangle::build(sides);

        assert!(triangle.is_none());
    }

    #[test]
    #[ignore]
    fn one_length_zero_side_first() {
        let sides = [0, 2, 2];

        let triangle = Triangle::build(sides);

        assert!(triangle.is_none());
    }

    #[test]
    #[ignore]
    fn one_length_zero_side_second() {
        let sides = [2, 0, 2];

        let triangle = Triangle::build(sides);

        assert!(triangle.is_none());
    }

    #[test]
    #[ignore]
    fn one_length_zero_side_third() {
        let sides = [2, 2, 0];

        let triangle = Triangle::build(sides);

        assert!(triangle.is_none());
    }

    #[test]
    #[ignore]
    fn equilateral_triangles_have_equal_sides() {
        let sides = [2, 2, 2];

        let triangle = Triangle::build(sides).unwrap();

        assert!(triangle.is_equilateral());

        assert!(!triangle.is_scalene());
    }

    #[test]
    #[ignore]
    fn larger_equilateral_triangles_have_equal_sides() {
        let sides = [10, 10, 10];

        let triangle = Triangle::build(sides).unwrap();

        assert!(triangle.is_equilateral());

        assert!(!triangle.is_scalene());
    }

    #[test]
    #[ignore]
    fn isosceles_triangles_have_two_equal_sides_one() {
        let sides = [3, 4, 4];

        let triangle = Triangle::build(sides).unwrap();

        assert!(!triangle.is_equilateral());

        assert!(triangle.is_isosceles());

        assert!(!triangle.is_scalene());
    }

    #[test]
    #[ignore]
    fn isosceles_triangles_have_two_equal_sides_two() {
        let sides = [4, 4, 3];

        let triangle = Triangle::build(sides).unwrap();

        assert!(!triangle.is_equilateral());

        assert!(triangle.is_isosceles());

        assert!(!triangle.is_scalene());
    }

    #[test]
    #[ignore]
    fn isosceles_triangles_have_two_equal_sides_three() {
        let sides = [4, 3, 4];

        let triangle = Triangle::build(sides).unwrap();

        assert!(!triangle.is_equilateral());

        assert!(triangle.is_isosceles());

        assert!(!triangle.is_scalene());
    }

    #[test]
    #[ignore]
    fn isosceles_triangles_have_two_equal_sides_four() {
        let sides = [4, 7, 4];

        let triangle = Triangle::build(sides).unwrap();

        assert!(!triangle.is_equilateral());

        assert!(triangle.is_isosceles());

        assert!(!triangle.is_scalene());
    }

    #[test]
    #[ignore]
    fn scalene_triangle_has_no_equal_sides_one() {
        let sides = [3, 4, 5];

        let triangle = Triangle::build(sides).unwrap();

        assert!(!triangle.is_equilateral());

        assert!(!triangle.is_isosceles());

        assert!(triangle.is_scalene());
    }

    #[test]
    #[ignore]
    fn scalene_triangle_has_no_equal_sides_two() {
        let sides = [5, 4, 6];

        let triangle = Triangle::build(sides).unwrap();

        assert!(!triangle.is_equilateral());

        assert!(!triangle.is_isosceles());

        assert!(triangle.is_scalene());
    }

    #[test]
    #[ignore]
    fn scalene_triangle_has_no_equal_sides_three() {
        let sides = [10, 11, 12];

        let triangle = Triangle::build(sides).unwrap();

        assert!(!triangle.is_equilateral());

        assert!(!triangle.is_isosceles());

        assert!(triangle.is_scalene());
    }

    #[test]
    #[ignore]
    fn scalene_triangle_has_no_equal_sides_four() {
        let sides = [5, 4, 2];

        let triangle = Triangle::build(sides).unwrap();

        assert!(!triangle.is_equilateral());

        assert!(!triangle.is_isosceles());

        assert!(triangle.is_scalene());
    }

    #[test]
    #[ignore]
    fn sum_of_two_sides_must_equal_or_exceed_the_remaining_side_one() {
        let sides = [7, 3, 2];

        let triangle = Triangle::build(sides);

        assert!(triangle.is_none());
    }

    #[test]
    #[ignore]
    fn sum_of_two_sides_must_equal_or_exceed_the_remaining_side_two() {
        let sides = [1, 1, 3];

        let triangle = Triangle::build(sides);

        assert!(triangle.is_none());
    }

    #[test]
    #[ignore]
    #[cfg(feature = "generic")]
    fn scalene_triangle_with_floating_point_sides() {
        let sides = [0.4, 0.6, 0.3];

        let triangle = Triangle::build(sides).unwrap();

        assert!(!triangle.is_equilateral());

        assert!(!triangle.is_isosceles());

        assert!(triangle.is_scalene());
    }

    #[test]
    #[ignore]
    #[cfg(feature = "generic")]
    fn equilateral_triangles_with_floating_point_sides() {
        let sides = [0.2, 0.2, 0.2];

        let triangle = Triangle::build(sides).unwrap();

        assert!(triangle.is_equilateral());

        assert!(!triangle.is_scalene());
    }

    #[test]
    #[ignore]
    #[cfg(feature = "generic")]
    fn isosceles_triangle_with_floating_point_sides() {
        let sides = [0.3, 0.4, 0.4];

        let triangle = Triangle::build(sides).unwrap();

        assert!(!triangle.is_equilateral());

        assert!(triangle.is_isosceles());

        assert!(!triangle.is_scalene());
    }

    #[test]
    #[ignore]
    #[cfg(feature = "generic")]
    fn invalid_triangle_with_floating_point_sides_one() {
        let sides = [0.0, 0.4, 0.3];

        let triangle = Triangle::build(sides);

        assert!(triangle.is_none());
    }

    #[test]
    #[ignore]
    #[cfg(feature = "generic")]
    fn invalid_triangle_with_floating_point_sides_two() {
        let sides = [0.1, 0.3, 0.5];

        let triangle = Triangle::build(sides);

        assert!(triangle.is_none());
    }
}
