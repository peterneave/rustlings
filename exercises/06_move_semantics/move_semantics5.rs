// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

/*
Only one borrower at a time
   |
14 |     let y = &mut x;
   |             ------ first mutable borrow occurs here
15 |     let z = &mut x;
   |             ^^^^^^ second mutable borrow occurs here
16 |     *y += 100;
   |     --------- first borrow later used here
*/

#[test]
fn main() {
    let mut x: i32 = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
