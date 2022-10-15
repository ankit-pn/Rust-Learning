use std::io;

fn main() {
    // overflow throw error
    // let x:u8 = 256;
    // let y:i8 = 10;
    let _x:u8 = 10;
    let _y:i8=20;

    // we cant add two number of different type 
    // let z= x+y;
    // println!("{}",z);

    // and if it is float then we have to write .f also


    // again overflow occur and compiler will give error accordingly.
    let p:u8 = 220;
    let q:u8 = 40;
    // let z= p+q;
    let z= p/q;
    println!("{}",z);

    // result of arthematic operation that we are gonna perform is of same type of operand that we have

    // otherwise define float to get accurate result
    // defalut type of float is f64

    // -----------type conversion and casting---------------
    let _t=100i8;
    let t1=100_i8; // underscore do the same thing ans 127_000 === 127000
    println!("{}",t1);

    let p1=100i64;
    let p2=2i32;
    // we have to manually convert type in rust
    // as keyword is used for that
    let r1=p1/p2 as i64;
    println!("{}",r1);

    // to get max value we use i32::MAX


    // taking input conversion
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Expecting a number");
    let iinput : i32 = input.trim().parse().unwrap();

    println!("{}",iinput+5);




}
