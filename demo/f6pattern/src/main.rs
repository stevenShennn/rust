fn main() {
   let x =4 ;
    let y = false;
    match x {
        4 | 5 if y => println!("yes"),
        _ => println!("no"),
    }
}


// demo1 :更强大的解构
struct Point {
    x: i32,
    y: i32,
}

// 解析一个结构体出来
//      fn main() {
//     let point  = Point {x: 0, y: 7};
//     match  point {
//         Point {x,y} => println!("({},{})",x,y),
//     }


// 对结构重命名
//fn main() {
//     let point  = Point {x: 0, y: 7};
//     match  point {
//         Point {x:x1,y:y1} => println!("({},{})",x1,y1),
//     }
// }


// 对特定字段进行解构
//fn main() {
//     let point  = Point {x: 0, y: 7};
//     match  point {
//         Point {x:x1,..} => println!("({})",x1),
//  }


// 忽略 和内存管理
//fn main() {
//
//     let tuple  :(u32,String) = (5,String::from("five"));
//
//     let (x,y) = tuple;
//     // 下面是会报错
//     //println!("x = {},y = {}",tuple.0,tuple.1);
//
//     let tuple = (5,String::from("five"));
//
//     let (x,_) = tuple;
//     println!("x = {}",x);
// }
//  总结一下，我们遇到了两种不同的模式忽略的情况——_和..。这里要注意，
// 模式匹配中被忽略的字段是不会被move的，而且实现Copy的也会优先被Copy而不是被move


// 范围和多重匹配
//fn main() {
//     let x =10 ;
//     match x {
//         1..=10 => println!("one through ten"),
//         _ => println!("anything"),
//     }
//
//     let c ='w';
//     match c {
//         'a'..='j' => println!("early ASCII letter"),
//         'k'..='z' => println!("late ASCII letter"),
//         _ => println!("something else"),
//     }
// }


// 多重匹配
// fn main() {
//     let x =1;
//     match x {
//         1 | 2 => println!("one or two"),
//         3 => println!("three"),
//         _ => println!("anything"),
//     }
// }

// ref 和 ref mut
//fn main() {
//     let mut x = 5;
//     match x {
//         mut mr => {
//             println!("Got a mutable reference to {}", mr);
//             mr = 6;
//             println!("Changed the mutable reference to {}", mr);
//         }
//     }
// }


// 通过@绑定值
//fn main() {
//     let x = 1u32;
//     println!("x = {}",x);
//     match x {
//         e @ 1 ..= 5 => println!("got a range element {}",e),
//         _ => println!("anything"),
//     }
// }

// 后置条件
//fn main() {
//    let x =4 ;
//     let y = false;
//     match x {
//         4 | 5 if y => println!("yes"),
//         _ => println!("no"),
//     }
// }