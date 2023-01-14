use triangle::Triangle;

fn main() {
    let sides = [2, 2, 2];

    let triangle = Triangle::build(sides);

    assert!(triangle.is_some());
}
