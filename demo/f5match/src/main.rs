use crate::Direction::South;

fn main() {
    let dire = Direction::North;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
        _ => println!("West"),// default
    };
}


enum Direction {
    North,
    South,
    East,
    West,
}



// return
fn  get_direction(dire:Direction)->String{
   let res = match dire {
        Direction::East => "East".to_string(),
        Direction::North | Direction::South => {
           panic!("South or North")
        },
        _ => "West".to_string(),// default
    };
    res
}

// demo2
enum Action {
    Sell(String),
    Buy(String),
}


fn demo2 (action:Action) -> String {
    let res = match action {
        Action::Sell(s) => String::from("se"),
        Action::Buy(s) => format!("Buy {}",s),
    };
     res
}