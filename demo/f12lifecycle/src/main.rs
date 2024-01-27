fn main() {
    println!("Hello, world!");
}


//  Owner: 资源的所有者 a
// Borrower: 资源的借用者 x
// Scope: 作用域，资源被借用/引用的有效期


// fn main() {
//     let a = 100_i32;
//
//     {
//         let x = &a;
//     }  // x 作用域结束
//     println!("{}", x);
// }

// 上面显示则是一个严重错误



// 1. 隐式lifetime
// fn foo(x: &str) -> &str {
//     x
// }


// 2. 显式的lifetime
//fn foo(x: &str, y: &str) -> &str {
//     if true {
//         x
//     } else {
//         y
//     }
// }
// 上main的会报错 ：必须支出对应的生命周期
// fn foo<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if true {
//         x
//     } else {
//         y
//     }
// }