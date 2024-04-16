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

fn main() {
    for_pattern_learning();
}
