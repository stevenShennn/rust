fn main() {
   let x = 5;
    {
        let x =12 ;
        assert_eq!(x,5);
    }

    assert_eq!(x,12);

    let x =42;
    println!("x = {}",x);
    // 变量存在遮蔽
    // 1. 作用域内的变量会遮蔽外部的变量
    // 2. 同一作用域内的变量会遮蔽之前的变量
}




//    // 1. 变量只能在初始化后才能被使用
//     let  x: i32; // 未初始化 被使用
//     let y: i32; // 未初始化 未被使用
//     println!("x: {}", x); // error[E0381]: borrow of possibly-uninitialized variable: `x`
//     // 报错 ： x被使用了，但是没有初始化




//     // 2. 变量只能被初始化一次
//   let __ =1 ; // __ 是一个合法的变量名
//     __ += 2;
//     println!("x = {}", __);



//    // 3. 作用域
// let x =1 ;
//     {
//          let y = 2;
//          println!("x = {}", x);
//     }
//     println!("x = {},y ={}", x,y);
//     // 这里会报错，因为y不在作用域内