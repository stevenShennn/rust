fn main() {

    // 当你指明具体数据类型，编译器会根据你的数据类型进行类型推断
    // i64
    let x =5;
    let y:i64 =6;
    let z:i64 = i64::MAX ;
    let m:i64 = i64::MIN ;
    println!("x ={},y= {} ,z= {} ,m= {} ",x,y,z,m);

    // u64
    let u64_x:u64 =1;
    let u64_y:u64 = u64::MAX ;
    println!("u64_x ={},u64_y= {} ",u64_x,u64_y);


    // i32
    let i32_x = 5;
    let i32_y:i32 = 6;
    println!("i32_x ={},i32_y= {} ",i32_x,i32_y);

    // u32
    let u32_x:u32 =1 ;
    let u32_y:u32 = u32::MAX ;
    let u32_z:u32 = u32::MIN ;
    println!("u32_x ={},u32_y= {} ,u32_z= {} ",u32_x,u32_y,u32_z);


    // i16
    let i16_x = 5;
    let i16_y:i16 =1;
    println!("i16_x ={},i16_y= {} ",i16_x,i16_y);

    // u16
    let u16_x:u16 =1 ;
    let u16_y:u16 = u16::MAX ;
    let u16_z:u16 = u16::MIN ;
    println!("u16_x ={},u16_y= {} ,u16_z= {} ",u16_x,u16_y,u16_z);


    // 基础成员变量
    let max = i64::MAX;
    let min = i64::MIN;
    let bits = i64::BITS;
    println!("max ={},min= {} ,bits= {} ",max,min,bits);


    // float
    // f32
    let f32_x = 1.0;
    let f32_y:f32 = 1.0;
    let f32_z:f32 = f32::MAX;
    let f32_m:f32 = f32::MIN;
    println!("f32_x ={},f32_y= {} ,f32_z= {} ,f32_m= {} ",f32_x,f32_y,f32_z,f32_m);

    // f64
    let f64_x = 1.0;
    let f64_y:f64 = 1.0;
    let f64_z:f64 = f64::MAX;
    let f64_m:f64 = f64::MIN;
    println!("f64_x ={},f64_y= {} ,f64_z= {} ,f64_m= {} ",f64_x,f64_y,f64_z,f64_m);


    // size
    let size_x:isize = 1;
    let size_y:isize = isize::MAX;

    // usize
    let usize_x:usize = 1;
    let usize_y:usize = usize::MAX;
    println!("size_x ={},size_y= {} ,usize_x= {} ,usize_y= {} ",size_x,size_y,usize_x,usize_y);

}


// 1. 存在长度： 8、16、32、64、size
// 2. 存在符号： i、u、f(f 没有size)