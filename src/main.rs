fn if_let_learning(){
    let favorite_color :Option<&str> = None;
    let we_are_tuesday =false;
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


fn main() {
    if_let_learning();
}
