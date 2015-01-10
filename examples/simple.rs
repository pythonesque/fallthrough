#[macro_use] extern crate fallthrough;

#[allow(unreachable_code)]
fn main() {
    let mut x = 0;

    match_fallthrough!(x, {
        0 => { assert_eq!(x,0); x = 1; },
        1 => { assert_eq!(x,1); x = 2; break; },
        _ => { panic!("Should not reach the default case"); },
    });
    assert_eq!(x, 2);
}
