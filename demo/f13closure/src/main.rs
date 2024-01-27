fn main() {



    // 闭包
    let a = |x :i32| x+1 ;
   // assert_eq!(2,a(2));

    // 复杂闭包
    let b = |x : i32| {
        let mut a = 1;
        return a +x
    };

    // 存在返回的闭包

    let c =|x :i32| -> i32{
        let mut d = 1;
        return d + x ;
    };

}


// 1. 闭包
//    let a = |x :i32| x+1 ;
//     assert_eq!(2,a(2))


//    // 复杂闭包
//     let b = |x : i32| {
//         let mut a = 1;
//         return a +x
//     };
//

// // 存在返回的闭包
//
//     let c =|x :i32| -> i32{
//         let mut d = 1;
//         return d + x ;
//     };


// 闭包作为参数返回值
fn call_with_one <F>(some_closure: F) ->i32
    where F:Fn(i32) ->i32 {
    some_closure(1)
}