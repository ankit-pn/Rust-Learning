fn main() {
    let mut x=5;// implicit 
    let _y: u32 = 4; //u-unsigned integer type
    // println!(x);
    // println!(y);
    println!("x is {}",x);

    // x=5; // not allowed
    x=3;
    // writing rust program is just great.
    println!("x is {}",x);

    // now changing value without making it mutable

    let y=5;
    println!("y is {}",y);
    let y=6;
    println!("y is {}",y);

    // name shadowing guide
    {
        let x=2;
        println!("x now is {}",x);
    }
    let x=x+1;
    println!("x now is {}",x);
    // if we are redefining we can chang typr also
    let x="hello";
    println!("x now is {}",x);

    // const
    const SECONDS_IN_MINUTE:u32 = 60;
    println!("{}",SECONDS_IN_MINUTE);
}
