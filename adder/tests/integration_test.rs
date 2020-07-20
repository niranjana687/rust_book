use adder;
mod common;
#[test]
fn it_adds_two() {
    assert_eq!(4, adder::adder_two(2));
}