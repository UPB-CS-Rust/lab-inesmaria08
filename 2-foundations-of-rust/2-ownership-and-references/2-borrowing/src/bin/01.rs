//! Make me compile only by reordering the lines in `main()`, but without
//! adding, changing or removing any of them.

fn main() {
    let mut x = 100;
    assert_eq!(x, 1200);
    let y = &mut x;
    *y += 100;
    let z = &mut x;
    *z += 1000;
}
