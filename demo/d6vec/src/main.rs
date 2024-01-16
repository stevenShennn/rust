fn main() {

    // vec
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6);
    println!("v = {:?}", v);


    // 通过下标
    let mut v1:Vec<i32> = vec![1,2,4];
    println!("v1[0] = {}", v1[0]);
    for i in &v1 {
        println!("i = {}", i);
    }

    for i in &mut v1 {
        *i += 1;
        println!("iiiii = {}", i);
    }


    // vec2
    let mut w2 = vec![1,2,3];
    let mut w3 = Vec::new();
    w3.push(1);
    w3.push(2);
    println!("w2 = {:?}", w3);
    w2.push(1);
    println!("w2 = {:?}", w2);


    // a1
    let mut a1:Vec<i32> = vec![1,2,3];
    let mut a2:Vec<&str> = Vec::new();

    a1.push(1);
    a2.push("1");

    println!("a1 = {:?}", a1);
    println!("a2 = {:?}", a2);
}
