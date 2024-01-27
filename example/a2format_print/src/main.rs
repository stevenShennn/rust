

// 这里是比较优雅的打印
/*#[derive(Debug)]
struct Person<'a>{
    name: &'a str,
}

fn main(){
    let name = "Peter";
    let petrer= Person{name:name};
    println!("{:?}",petrer);
}*/


struct UnPrintable(i32);


#[derive(Debug)]
struct DebugPrintable(i32);


fn main (){
    println!("{:?}"
}