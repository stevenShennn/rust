fn main() {
   // slice
    let b:&[i32] = &[1,2,3];
    println!("{:?}",b);

    // 数组转化成为slcie
    let a:[i32;3] = [1,2,3];
    let b:&[i32] = &a;
    println!("{:?}",b);


    // for
    for i in b {
        println!("i={}",i);
    }

    // slice [a..b] 从a开始到b结束，不包含b
    let c:&[i32] = &b[2..3];
    println!("{:?}",c);
}
