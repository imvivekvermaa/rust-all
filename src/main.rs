//#![allow(unused_variables)]
const _TESTING_CONSTANT: &str = "hello";
fn main() {
    //println!("Hello, world!");
    //all_available_integers();
    //all_float_things();
    //practice();
    //let result: bool = is_even(7);

    //println!("{result}");

    //conditionals();
    //loops()
    ownership_concepts();
    strings_example();
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
    ); //this :.1 inside {} is called float format specifier
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

    //STRINGS

    println!("this is string literal.");
    println!(
        "this is string literal and \" \\ \" is an special char and using it with diff characters we will see what we can do."
    );
    println!("hello and \\n creates a new line \n like here we did.");
    println!("similarly \\t creates spaces of 4 same as a tab.");
    println!("we can do \\\" \\\", in order to use double quotes within the string");
    println!("similarly \\\\ incase we need \\ inside a string like for path of something.");

    // Also there's something called raw srtring
    // so raw string make all of above dynamic things just plain string
    // like no \n \t \ or \" will work.
    // but we still can't simply do "" inside a string. it's invalid, so we use \" if n only
    // if the string is not raw string.

    println!(r"this is a way of creating a raw string and we can use inside it.");

    // There are built-in rust methods like there are in any other languages
    // on strings, floats, integer and on other datatypes
    // need to check 'em out!
    //like float values has .ceil, .floor, .round. same there'll be on strings etc.

    //Type Casting with "as" keyword!
    //this below examples is casting diff types

    let _pp: i32 = 2345;
    let _bb = _pp as i8;

    println!("{}", "coke" == "coke");

    //array
    let array_example: [&str; 4] = ["apple", "nuts", "cat", "hooter"];
    let another_example: [i32; 0] = []; //this is how we define empty array
    //incase of dynamic  values;
    //but still you can't change/mutate the elements values inside an array
    //without making it mutable
    let mut mutable_array: [&str; 3] = ["test1", "test2", "test3"];

    println!("{} is the mutable array.", mutable_array[1]);

    mutable_array[1] = "testing";
    println!("{} is mutated array.", mutable_array[1]);

    //tuples
    let employee = ("molly", 32, "sales");

    let name: &str = employee.0;
    let age: i32 = employee.1;
    let dept: &str = employee.2;
    // we could also do simple destructuring of tupele like let (name, age, dept) = employee;
    println!("{} is name", name);
    println!("{age} is age");
    println!("{dept} is department");
    println!("{employee:?} is is whole tuple here using debug trait w format specifier");
    println!(
        "{employee:#?} is is whole tuple here using debug trait w format specifier but along w pretty printing using #"
    );
}

fn is_even(nmbr: i32) -> bool {
    if nmbr % 2 != 0 {
        return false;
    }

    return true;
}

fn conditionals() {
    let a: &str = "hi";
    let boolean: bool = true;
    let num: i32 = 4;

    //there is no concept of truthiness values in rust the outcome/comparision
    //on condition should be in pure form of boolean (true/false)
    /*if a {
        println!("this code block will not run since condition is not in boolean form.");
    }

    if num {
        println!("this will also not work since the condition is not boolean and just a number.");
    }
    */

    if a == "hi" {
        println!(
            "this will get printed since the comparision will produce a boolean at condition level."
        );
    }

    if boolean {
        println!("this will get printed since the condition is in boolean form");
    }

    //there no ternary operators in rust but we can do something look like

    let result = if a == "hii" {
        println!("result was true")
    } else {
        println!("the result was false")
    };

    let check_number = 5;

    match check_number {
        5 => {
            println!("this code will run")
        }
        7 => {
            println!("if a is 7 then this code will run")
        }
        _ => {
            println!("this is default case")
        }
    };

    //we can also do the match statements like this if not multiple line need to be there.
    //always keep _ (default) at the end of all match arms
    //since once _ occure the further execution won't happend after it.

    match check_number {
        5 => println!("this code will run"),
        7 => println!("if a is 7 then this code will run"),
        _ => println!("this is default case"),
    };

    //weird but we can use if statements inside a match

    match check_number {
        value if value % 2 == 0 => println!("{value} is even."),
        value if value % 2 == 1 => println!("{value} is odd."),
        _ => unreachable!(), // unreachable macro means telling rustc that
                             // this line of code will never run and if it ever does then
                             // panic immediately.
                             // _ => println!("Unknown")

                             //although both of above lines will cover all the scenarios but
                             //rsut still won't figure out the what if case so default arm must be there!, Hence-
    }
}

//this is loop example with break statement and similarly there's continue just like in other
//languages
fn loops() {
    let mut count: i32 = 0;
    let mut count1: i32 = 0;
    let count2: i32 = 10;

    loop {
        println!(
            "this will loop to infinitely and {} is the count value.",
            count
        );
        count += 1;

        if count >= 10 {
            break;
        };
    }

    while count1 < 10 {
        println!("this is count: {}", count1);

        count1 += 1;
    }

    //1..count2 means less then count and 1..=count2 is till the count2 mean equal
    //we can use integers or alphabates like 'a'..='z' and it will iterate 26 characters
    for i in 1..=count2 {
        println!("{i} is the changing digit in here using for loop taste with range.");
    }
}

fn ownership_concepts() {
    // a is owner of value 55 and whenever the scope of a will go out of scope then
    // variable a will clean the memory of the value 55 form heap or stack. in this case
    // from stack.
    let a: i16 = 55;
    println!("{a}");

    {
        // as soon as this block will end the b variable shoyld clean the memory
        // since b is owner of this string value and hence we won't be able to
        // access b outside of this block.
        let b: char = 'A';
        println!("{b}");
    }

    /*for context, Traits are just kind of contacts which diff "types AKA datatypes" promise to follow/have.

    there's a copy trait and all of the data types which goes to stack memory have "copy" trait
    which means: 
    */

    let number1: i16 = 23;

    let another_number =  number1;
    //it has whole another 23 value in stack separately which has nothing to do with number

    println!("{number1}");
    println!("{another_number}");

}


fn strings_example() {
    // till now i have declared strings data variables with type &str
    // but there are other ways as will
    // this &str ways is neither on stack nor on heap memory all_float_things
    // when we do &str, it means that it's just static view of utf-8 text mutable_array
    // exist somewhere in compiler or system and it's already in bits so no memory all_float_things
    let one_way: &str = "hello world";
    let second_way: String = String::new(); // this we use when the string is dynamic
                                            // like no hardcoded value at compilation time
                                            // like in case of user inputes and other creates

    //this below String::from("hollow world") is very interesting
    //in most raw explaination it like we create a string with some existing value in it
    //allocating the memory in heap and referencing the address to the value.
    //also that string is growable/mutable as well.
    // third_way = "hollow world is weird place".to_string(); if we make third_way mutable wit mut
    let mut third_way: String = String::from("hollow world");

    //that's how we append some more text to string, it will all be in heap
    third_way.push_str("all around");

    // There's a concept of ownership which is valid on heap data references
    let sample_string: String = String::from("ownership checking happening");
    println!("{sample_string}");

    //this is important! so waht clone does is that we're explitly enforcing rust to create a copy
    //of same sample_string value in heap again, it's just like integers and other things
    //so since now sample_string_new_owner has his own value pointer/reference in heap for the
    //value that means the code works fine.
    //
    //important note: drop() is made of 2 steps (move + destruction of value)
    //
    let sample_string_new_owner = sample_string.clone();

    // println!("{sample_string}"); this will throw error because sample_string
    // doesn't have ownership anymor:wille. the ownership of reference has been moved to
    //sample_string_new_owner

    drop(sample_string);

    println!("{sample_string_new_owner}");

    println!("{one_way}");
    println!("{second_way}");
    println!("{third_way}");


    // moer examples
    // here with &mut the ownership has not been moved instead  testing_references_and_borrowing
    // bowrrowed the mutatble reference from sample, mutating the string and that's it.
    // ownership is still in sample variable's hand.

    let mut sample: String = String::from("hello there");
    testing_references_and_borrowing(&mut sample);

    println!("{sample}");

    multiple_ref_restrictions();

    dangling_ref();

    ownership_w_arrays_tuples();


}

fn testing_references_and_borrowing(sample: &mut String) {
    sample.push_str(", we just mutate the string using mut ref");
    println!("here's the {sample}");

}

fn multiple_ref_restrictions() {
    //In rust we can have as many immutable refs as we want but
    //we cannot have mutliple mutable refs or a mutable and other immutable
    //refs side by side at all! for examples

    let mut check1: String = String::from("Apple");
    let ref1_check: &mut String = &mut check1;
    
    //here below it will work because ref2_check variable is in no existence till here.
    //this is also about something call lifetimes in rust.
    println!("ref1_check: {ref1_check}");
    let ref2_check: &String = &check1;

    //this below line throw error as per what we said above for usid sbs.
    //println!("ref1_check: {ref1_check} and ref2_check: {ref2_check}");

    //println!("ref1_check: {ref1_check}"); //this wiil not work since it's after ref2_check
    println!("ref2_check: {ref2_check}");
}

fn dangling_ref() { // -> &String will throw error
    let sample_string: String = String::from("test");
    println!("sample_string: {sample_string}");

    //we know variables are destroyed along w their allocated memory after their scope
    //here the scope is this function so technically sample_string will be gone once
    //this function will be done but what if i return the referenceof sample_string from this
    //function and use it inside main func?
    //the pointer/reference will make no sense right! it will be a dangling ref pointing to a mem
    //address which doesn't have string data 
    //
    //so rust compiler don't let us do that and throw error when we ruturn the reference
    //btw references are also some value as we know right using & and actual values using *
    // so yeah


    // &sample_string

}

fn ownership_w_arrays_tuples() {
    let int_array: [i32; 3] = [3, 2, 5];
    let first_ele: i32 = int_array[0];

    println!("first_ele: {first_ele}");

    //here just like above will not work
    //reason is simple remember copy trait with int tyes so above is already a copy create and
    //ownerships has not been moved but inscase of strings array
    //now that i did first_ele1 to 0th value of array the the ownership
    //has partially moved. so either we need to  use .clone() method or
    // & to make a reference.

    //let string_array: [String; 3] = ["apple", "ball", "puc"];
    //let first_ele1: String = string_array[0];

    let string_array: [String; 3] = [String::from("apple"), String::from("ball"), String::from("puc")];
    let first_ele1: &String = &string_array[0];


    println!("first_ele1: {first_ele1}");
}
