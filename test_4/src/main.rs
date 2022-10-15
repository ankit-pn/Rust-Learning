// this is to learn data types in rust

fn main() {
    // primitive data types
    let x: i32 = 2;
    // by defalut it is i32
    // we have i8,i16,i32,i64,i128;
    // they are signed interger
    let y: u32 = 932;
    // u32 is unsigned
    // u8,u16.... every thing exist
    //
    let p: f32 = 10.62;
    
    // f32 and f64 exist

    let true_or_false: bool = true; //0 and 1 are not used.
    let letter: char = ';';


    // tupple
    let tup1 : (i32,bool,char)= (1,true, 'x');
    let tup2 : (i8,bool,char)= (1,true, 'x');

    // tup1 and tup2 are not same

    println!("{},{},{},{},{}",x,y,p,true_or_false,letter);
    println!("{}",tup1.1);
    println!("{}",tup2.2);

    // mutating some more code...

    let mut tup3: (i32,bool,char) = (1,true,'s');
    println!("Before mutating {}",tup3.2);
    tup3.2='p';
    println!("After mutating {}",tup3.2);

    // if tup is mut then we can element
    // array also exist
    let arr1 = [1,2,3,4,5];
    println!("{}",arr1[3]);

    // how to type arr type 

    let arr2: [i32;5] = [1,2,3,4,5];
    println!("{}",arr2[2]);
}
