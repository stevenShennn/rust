use std::ops::Add;

fn main() {
    println!("Hello, world!");
}


// 泛型 ：泛型本质就是多态的一种，只是在具体的执行层面不需要管理你是什么类型

enum Option<T> {
    Some(T),
    None,
}

// 简单泛型
/*fn demo (){
    let a: Option<T> = Some(1001);
    let b:Option<T> = Some("hhhhh");

    // 也可以指定a 的T的类型
    let c:Option<f32> = Some(0.21);
}*/

//一个简单的加法函数

/*fn add_generics <T:Add<T,Output= T>> (a:T,b:T)->T{
    a+b
}*/



// 让自定义类型支持Add

/*struct  Point {
    x :i32,
    y:i32
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point{
            x:self.x + rhs.x,
            y:self.y + rhs.y,
        }
    }
}*/