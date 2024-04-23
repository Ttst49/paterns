use std::io;

#[allow(unused)]
fn if_let_learning(){
    let favorite_color :Option<&str> = None;
    let we_are_tuesday = false;
    let age: Result<u64,_> = "34".parse();

    if let Some(color) = favorite_color{
        println!("Using your favorite color {} as background",color);
    }else if we_are_tuesday {
        println!("Tuesday it's green day")
    }else if let Ok(age) = age{
        if age>30 {
            println!("Using purple as background")
        }else {
            println!("Using orange as background")
        }
    }else {
        println!("Using blue as background")
    }
}

#[allow(unused)]
fn while_let_learning(){
    let mut pile = Vec::new();
    pile.push(1);
    pile.push(2);
    pile.push(3);

    while let Some(top_data) = pile.pop() {
        println!("{}",top_data);
    }
}


#[allow(unused)]
fn for_pattern_learning(){
    let v = vec!["a","b","c"];

    for (index,value) in v.iter().enumerate(){
        println!("{} is at index: {}",value,index);
    }
}

fn matching_literals(){
    let mut input = String::new();
    println!("Enter an integer:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let x= input.trim().parse().expect("Wrong data");

    match x {
        1=>println!("one"),
        2=>println!("two"),
        3=>println!("three"),
        _=>println!("anything else")
    }
}


fn matching_with_masked_variable(){
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("We got 50"),
        Some(y) => println!("We got y = {}",y),
        _=>println!("Default case: {:?}",x)
    }

    println!("At the end, x={:?} and y={:?}",x,y)

}

fn main() {
    matching_with_masked_variable();
}
