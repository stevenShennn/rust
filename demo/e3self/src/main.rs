fn main() {
    // impl 存在三种self

    // 1. self ： 代表所有权
    let s1 = Self1::new(10);
    println!("s1 age is {}",s1.get_age());
    // println!("s1 age is {}",s1.age) : error: field `age` of struct `Self1` is private
    // 这里因为age 被访问了，所以s1的所有权被转移了，所以s1不能再使用了，结构体本身就不能使用了
    // 解决方案：使用引用
    // 2. 使用copy
   //  println!("s1 age is {}",s1.age); 但是这里存在一个问题，copy出来的结构体你是不能修改的，因为是不可变的

    // 2. &self ： 代表共享引用
    let s2 = Self2::new(10);
    s2.greet();
    println!("s2 age is {}",s2.age);


    // 3. &mut self  ： 代表可变引用
    let mut s3 =Self3{age:10};
    s3.show();
    s3.add_one();
    s3.add_two();
    s3.show();


    // 总结一下：三种self的使用场景
    // 1.self ：代表所有权，一般用于构造函数，因为构造函数需要转移所有权
    // 2. &self：代表共享引用，一般用于读取数据
}



//# [derive(Copy, Clone)]
struct  Self1 {
    age: i32
}

impl Self1  {
   fn new(age:i32) -> Self1{
       Self1{
           age: age
       }
   }

    fn get_age(self) ->i32{
        self.age
    }
}


struct  Self2 {
    age: i32
}

impl Self2 {
    fn new(age:i32)->Self2 {
        Self2{
            age: age
        }
    }

    fn greet(&self){
        println!("hello {}",self.age);
    }

   /* fn change(&self){
        // self.age += 1; error: cannot assign to immutable field `self.age`
    }*/

}



struct Self3 {
    age : i32
}

impl Self3 {
    pub fn show(&self){
        println!("age is {}",self.age);
    }

    pub fn add_two (&mut self){
        self.age += 2;
    }

    pub fn add_one(&mut self){
        self.age += 1;
    }
}

