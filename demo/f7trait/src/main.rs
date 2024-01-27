
// 可以为内置类型定制一些功能

trait HasAge {
    fn age(&self)->i32;
}

trait ExtendHasAge:HasAge {
    fn foobar();
}

impl HasAge for i32 {
    fn age(&self) -> i32 {
        return *self
    }
}


fn main() {

}

// trait 与具体类型
//trait HasArea {
//     fn area(&self) -> f64;
// }
//
// struct Circle {
//     name : String,
// }
//
// impl HasArea for Circle {
//     fn area(&self) -> f64 {
//         0.01
//     }
//
// }


// trait 和泛型
//use std::fmt::Debug;
//
// fn foo<T:Debug>(s:T){
//     println!("{:?}",s)
// }


// 多trait 约束
//use std::fmt::Debug;
//
// fn foo<T:Debug +Clone>(s:T){
//     s.clone();
//     println!("{:?}",s)
// }



// where 关键字
//use std::fmt::Debug;
//
// fn foo<T:Debug +Clone>(s:T){
//     s.clone();
//     println!("{:?}",s)
// }
//
// // where 从句
// fn foo1<T,K>(x:T,y:K) where T:Clone,K:Clone+Debug{
//     x.clone();
//     y.clone();
//     println!("{:?}",y)
// }
//
// // 或者
// fn foo2<T,K>(x:T,y:K)
//     where T:Clone,
//             K:Clone+Debug{
//     x.clone();
//     y.clone();
//     println!("{:?}",y)
// }


// trait 和内置类型
//
// 可以为内置类型定制一些功能
//trait HasAge {
//     fn age(&self)->i32;
// }
//
// impl HasAge for i32 {
//     fn age(&self) -> i32 {
//         return *self
//     }
// }


// trait 默认方法
/*trait HasAge {
    fn age(&self)->i32;
    fn is_valid(&self) ->bool;
    fn is_invalid(&self) -> bool { !self.is_valid() }
}*/


// trait 继承
//// 可以为内置类型定制一些功能
//
// trait HasAge {
//     fn age(&self)->i32;
// }
//
// trait ExtendHasAge:HasAge {
//     fn foobar();
// }


// derive
// #[derive(Debug)]