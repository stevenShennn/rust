fn main() {
    let y = (1, "hello", 4.5, true);
    let x:(i32,&str) = (1, "hello");

    let (w,z) = x;
    println!("w = {}, z = {}", w, z);

    // 用下标
    let f =x.0; // f =3
    let s = x.1; // s = "hello"
}
