fn main() {

    // array : [type; size]
    let mut a = [[0; 3]; 3];
    a[0][0] = 1;
    a[1][1] = 1;
    a[2][2] = 1;
    println!("{:?}", a);

    // 看一维数组的内存布局
    let b:[i32;3] = [0,0,0];
    println!("{:?}",b);
    println!("b1={}",b[0]);


    let bstr:[&str;3] = ["a","b","c"];
    println!("{:?}",bstr);

    // 二维数组
    let bbs:[[&str;3];3]= [["a","1","1"],["a","1","1"],["a","1","1"]];
    println!("bbs1={:?}",bbs[1]);
    show(bbs[0]);
    show(bbs[1]);
    show(bbs[2]);

    // 二维数组f32
    let ffs:[[f32;1];2]= [[1.0],[2.1]];
    println!("{:?}",ffs)

}


fn  show(content:[&str;3]){
    for i in content {
        println!("content is {}",i)
    }
}


