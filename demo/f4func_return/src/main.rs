

// 返回为空的时候： -> ()
// 也可以省略
fn main()->(){
    println!("Hello, world!")
}


fn get_name() ->String{
    let name = "rust";
    return name.to_string();
}

// return 关键字
fn get_name_wrapper(name:&str) ->String{
    if name == "rust"{
        return "rust".to_string();
    }
    "not rust".to_string()
}


// 返回多个关键字
fn get_many_name()->(String,String){
    ("rust".to_string(),"rust".to_string())
}


// return 多个关键字
fn get_many_name_wrapper(name:&str)->(String,String){
    if name == "rust" {
        return ("小红".to_string(),"小明".to_string());
    }
    ("rust".to_string(),"rust".to_string())
}

// 发散函数 ：diverging 函数，没有返回，使用感叹号表示
// 发散函数的结束 不能有返回值 ，都是panic
fn diverging() ->! {
    panic!("hahha")
}


type Name = String;

type Fc = fn()->String;
