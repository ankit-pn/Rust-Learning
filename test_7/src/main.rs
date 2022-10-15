fn main() {
    // rust specific expression

    let number = {
        let x=3;
        x+1
    }; //statement and inside it we have expression, important to not put semicolon in it.
    println!("{}",number);
    let res:i32 = add_numbers(2,45);
    println!("{}",res);
}

// let see how to return value from function

fn add_numbers(x:i32,y:i32) -> i32{
    // we can also use return statement in it.
        x+y
}
