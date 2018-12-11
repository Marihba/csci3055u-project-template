use std::collections::HashMap;

fn main() {

    // introducing scopes represented by curly braces
    {
        let mut s = String::from("Programming");  // how strings are assigned to var in Rust
        s.push_str(" Languages!");     // a concantenation function to the above string
        println!("{}",s);
    }

    let st1 = String::from("Computer Science!");
    let st2 = st1;    //shallow copy, only updates pointer to the new variable
    println!("{}", st2);
    //println!("{}", s1); // will not work because this variable looses scope once passed to s2

    //-------------------------------------------
    let mut str = String::from("Abhiram");

    test_function(&mut str);

    //----------------------------------
    let str2 = String::from("This is a sentence!!");

    // splicing the sentence, done through reference!!
    let sentence = &str2[1..9]; //this is a range starting from 0 to 5, exclusively
    println!("sentence contains the word : \"{}\"", sentence);

    //---------------------------------------------------------------------------------------------
    // array collections in rust's standard library

    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12321];

    // for loop on array data collection
    for i in arr.iter() {       // iter is a simple but powerful tool that works well with for loops
        println!("The number at {} is now {}!", i, i*12);
    }

    // example of the vector data structure, and iteration of the indices
    let vec = vec![123, 432, 123, 545, 765];
    let mut num = 0;
    for i in vec.iter() {       // iter is a simple but powerful tool that works well with for loops
        println!("Index at {} is {}", num, i);
        num += 1;
    }

    // example of the HashMap data structure
    let mut biodata = HashMap::new();
    biodata.insert("James", 34);
    biodata.insert("Tim", 42);
    biodata.insert("Michael", 55);
    biodata.insert("Kobe", 40);

    // how to print information from the map 'contacts'
    for (name, age) in biodata.iter() {
           println!("Calling {}: {}", name, age);
       }
}

fn test_function(some_string: &mut String) {  // passing a reference type that is also mutable
    some_string.push_str(" is going to successfully complete this project!");
    println!("{}", some_string);    // end result of this function
}
