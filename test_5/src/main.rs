use std::io;


fn main() {
   // console input in rust

   // there is prelude, that is some of functionality that is automatically added to rust code.
   // that is because of which we are able to use println!() and fn
   // package in rust is often refered as crates

   // create input
    let mut input = String:: new();
    //taking input from console.

    io::stdin().read_line(&mut input).expect("failed to read line");


    // by reference we can diretly modify data inside the input variable
    // by deflaut we are passing a copy and if we have 
    // by that way it will not change the value 
    // reference by default is immutable
    // so we have to write &mut
    
    println!("{}",input);

}
