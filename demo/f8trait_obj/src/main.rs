use std::fmt::format;

fn main() {

    let v =vec![1,2,3];
    let o = &v as &Clone

}



//trait对象在Rust中是指使用指针封装了的 trait，比如 &SomeTrait 和 Box<SomeTrait>。
/*trait Foo { fn method(&self) ->String ;}

impl Foo for u8 {
    fn method(&self) -> String {
       format!("u8 : {}",*self)
    }
}


impl Foo for String {
    fn method(&self) -> String {
        format!("string: {}",*self)
    }
}

fn do_something(x: &Foo) {
    x.method();
}*/

//fn main() {
//     let x = "Hello".to_string();
//     do_something(&x);
//     let  y =8u8;
//     do_something(&x);
//
// }




// trait 对象实现
/*pub struct TraitObject {
    pub data: *mut (),
    pub vtable: *mut (),
}*/



// 对象安全

/*let v =vec![1,2,3];
let o = &v as &Clone*/