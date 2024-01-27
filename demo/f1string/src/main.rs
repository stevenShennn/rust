fn main() {

    // 字符串在rust中实际是被Utf-8编码的字节数组
    let x = "hello";
    let x:&'static str = "hello";
    println!("{}", x);

    // String

    let a:&'static str = "hello";

    let mut y:String = a.to_string();
    y.push_str(", world");
    println!("{}", y);

    // 将String 转换成 &str
    let z:&str = &*y;

    for i in z.as_bytes() {
        print!("{} ", i);
    }

    for i in z.chars() {
        print!("{} ", i);
    }

}


