use adder;
mod common;
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::adder_two(2));
}