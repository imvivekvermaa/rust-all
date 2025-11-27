//#![allow(unused_variables)]
const _TESTING_CONSTANT : &str = "hello";
fn main() {
    println!("Hello, world!");
    all_available_integers();
    all_float_things();
    practice();
}

fn all_available_integers() {
    let a: i8 = -23;
    let b: i16 = 11;
    let c: i32 = 1998;
    let d: i64 = 25;
    let e: i128 = 121970;

    let f: u8 = 23;
    let g: u128 = 111998;

    let h: isize = -25121998;

    println!(
        "these are all example of all signed integers {} {} {} {} {} ",
        a, b, c, d, e
    );
    //another way of capturing the dynamic values from a variable
    println!(
        "these are all example of all signed integers {0} {1} {1} {2} {3} {4}",
        a, b, c, d, e
    );

    println!("these are all example of all signed integers {a} {b} {c} {d} {e} ");

    println!(
        "these are all example of all unsigned integers {} {} ",
        f, g
    );
    println!(
        "this is example of integer size based on my machine's bit size architect {} ",
        h
    );
    println!(
        "isize {} is the bytes on my computer ",
        std::mem::size_of::<isize>()
    );
}

fn all_float_things() {
    let a: f32 = -23.123123;
    let b: f64 = 34.234;
    let c: f32 = 1.2321e3;

    println!("this {} is a float number here ", a);
    println!(
        "this is a way to get the determind float point to print! {:.1}",
        b
    );
    println!(
        "we can also use mathematic notations like 'e' in a float variable {}",
        c
    );
}

// practice

fn practice() {
    type Kms = i16;
    let mut fav_color: &str = "blue";
    // i used _ below but i could aslo use #[allow(unused_variables)] or i could have
    // written #![allow(unused_variables)] at the top. "!" is important while using the directive
    // at top
    let _intentionally_unused_v: i16 = 82;
    //this will throw an error because fav_color was not mutable so we need to make it mutable
    //at the time of defining the variable
    // also incase of any error, to resolve it just do = rustc --explain <error-code>
    // like in this case error code is E0384
    fav_color = "pink";

    println!("{} is my favorite color", fav_color);
    println!("{fav_color} is my favorite color");
    println!("{0} is my favorite color", fav_color);

    let shadowing: i16 = 21;
    println!("here the value will be original/integer as {}", shadowing);
    //using shadowing we also change the datatypes of a variable
    let shadowing: &str = "hello";
    println!(
        "this is called variable shadowing we just change the value from int (21) to {}",
        shadowing
    );
    
    let _race_track_lenght: Kms = 12;
    println!("this was an example for type alias here, in this case as 'Kms'.");

}
