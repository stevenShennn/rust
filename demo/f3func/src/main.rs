
// fn function
fn main() {
    let xh = ("小红",18);
    let xw = ("小王",20);

    print_id(xh);
    print_id(xw);

}


// 函数参数 param
fn say_hello(name:&str) {
    println!("Hello, {}!", name);
}

// 函数作为参数
fn say_hello_to(name:&str ,handler:fn(&str)){
    handler(&name);
}

// 模式匹配
fn print_id ((name,age):(&str,i32)){
    println!("name:{},age:{}",name,age);
}


fn print_id2((age,name):(i32,&str)){
    println!("name:{},age:{}",name,age);
}

