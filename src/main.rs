use std::io;

#[allow(unused)]
fn if_let_learning() {
    let favorite_color: Option<&str> = None;
    let we_are_tuesday = false;
    let age: Result<u64, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color {} as background", color);
    } else if we_are_tuesday {
        println!("Tuesday it's green day")
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as background")
        } else {
            println!("Using orange as background")
        }
    } else {
        println!("Using blue as background")
    }
}

#[allow(unused)]
fn while_let_learning() {
    let mut pile = Vec::new();
    pile.push(1);
    pile.push(2);
    pile.push(3);

    while let Some(top_data) = pile.pop() {
        println!("{}", top_data);
    }
}

#[allow(unused)]
fn for_pattern_learning() {
    let v = vec!["a", "b", "c"];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index: {}", value, index);
    }
}

#[allow(unused)]
fn matching_literals() {
    let mut input = String::new();
    println!("Enter an integer:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let x = input.trim().parse().expect("Wrong data");

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        4 => println!("four"),
        _ => println!("anything else"),
    }
}

#[allow(unused)]
fn matching_with_masked_variable() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("We got 50"),
        Some(y) => println!("We got y = {}", y),
        _ => println!("Default case: {:?}", x),
    }

    println!("At the end, x={:?} and y={:?}", x, y)
}

enum Color {
    Rgb(i64, i64, i64),
}

#[allow(unused)]
enum Message {
    Quit,
    Move { x: i64, y: i64 },
    Write(String),
    ChangeColor(Color),
}

#[allow(unused)]
fn using_message_enum() {
    let msg = Message::ChangeColor(Color::Rgb(134, 23, 27));

    match msg {
        Message::Quit => {
            println!("Quit variant can't be exploded")
        }
        Message::Move { x, y } => {
            println!("moving about {} on x-axis and about {} on y-axis", x, y);
        }
        Message::Write(text) => {
            println!("textual message: {}", text)
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Changing red value to {}, green to {} and blue to {}",
                r, g, b
            )
        }
        Message::ChangeColor(Color::Rgb(t, s, v)) => {
            println!(
                "Changing hue to {}, vividness to {} and brightness to {}",
                t, s, v
            )
        }
    }
}

#[allow(unused)]
fn matching_with_default() {

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("You can't override existing data");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting equal {:?}", setting_value);
}


#[allow(unused)]
fn use_only_what_needed(){
    struct Point{
        x:i64,
        y:i64,
        z:i64
    }
let origin = Point{x:0,y:12,z:32};
match origin{
    Point{x,..} => println!("x equal {}",x)
    }

}

#[allow(unused)]
fn wrong_double_dots_method(){
    //let numbers = (2,23,23,34,80);
//match numbers {
    //(..,second,..)=>{
        //println!("here are some numbers {}",second)
    //}
    //}
}

fn main() {
    matching_with_default();
}
