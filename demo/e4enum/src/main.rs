fn main() {
    let direction = Direction::Up;
    match direction {
        Direction::Up =>println!("is up"),
        Direction::Down =>println!("is down"),
        Direction::Left =>println!("is left"),
        Direction::Right =>println!("is right"),
    }

    let point = SpecialPoint::Point(1,2,3);
    match point {
        SpecialPoint::Point(x,y,z) =>{
            println!("x:{}",x);
            println!("y:{}",y);
            println!("z:{}",z);
        },
        SpecialPoint::Special(s) =>println!("s:{}",s),
    }



    let point2 = SpecialPoint2::Point{
        x:1,
        y:2,
        z:3
    };


    match point2 {
        SpecialPoint2::Point { x:x,y:y,z:z } => {
            println!("x:{}",x);
            println!("y:{}",y);
            println!("z:{}",z);
        }
        SpecialPoint2::Special(_) => {}
    }

}


// 都是大写字母开头
enum Direction{
    Up,
    Down,
    Left,
    Right
}

// 特殊枚举
enum SpecialPoint{
    Point(i32,i32,i32),
    Special(string)
}

// == 下面

enum SpecialPoint2{
    Point{
        x:i32,
        y:i32,
        z:i32
    },
    Special(string)
}