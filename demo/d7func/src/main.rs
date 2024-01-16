fn main() {
    let x:fn(i32) -> i32 = fo;
    assert_eq!(x(1), 2);
}



fn fo(x:i32)-> i32 { x + 1}