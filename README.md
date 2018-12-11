# Rusk Programming Language Research Project

- Abhiram Sinnarajah
- Abhiram.sinnarajah@uoit.net

## About the language

> _What is rust?_
> - From a beginners perspective, Rust's syntax gives a feeling as if it is a combination of C++ and Clojure syntax. The core of Rust language is functional programming. 
> - History:
Rust started off as a project in 2006 by a Mozilla employee, Graydon Hoare.Mozilla later sponsered the project in 2009. Rust was officially announced in 2010. 
> - Some interesting features:
A language that heavily focuses on safety and safe concurrency. Because of this, Rust is perfect for systems programming lanuage.The design allows users safer memory while mainting optimal performance. 

## About the syntax

> _basic syntax examples_

*Main entry point function, main*
```
fn main() {...} // empty main function with zero arguments passed
```

*Hello World on main*
```
fn main() {
  println!("Hello World");
}
// println! is a macro function for println
```

*Declaring and Assigning Variables*
```
// use let for variable assignment
let x = 10

```

*Declaring and Assigning Mutable Variables*
```
let mut x = 20;
```

*Collections: Declaring Tuple Variables*
```
let numTup: (i32, f32, i64) = (123, 1.23, 1.234);
let (a, b, c) = numTup;
```

*Collections: Arrays*
```
let brands = ["Toyota", "Nissan", "Suzuki", "Honda", "Acura", "Volkswagen"];
brands[2] // Suzuki
```

*Creating functions in Rust*
```
fn new_func() {
  println!("Hello there!");
}
```

*Functions with params/args*
```
// function with param x of type integer 32 bit
fn new_func(x:i32) {
  println!("Hello, {}", name)
}
```

*Invoking a function*
```
new_func(500);  // -> 500
```

## About the tools

Rust is an ahead-of-time compiled language. This means that source code represeneted by
the file_name.rs needs to be compiled in order for it to be able to run the code. The compiled code
is a binary ececutable. The machine runs this executable generated for the user. To compile a file, type rustc command followed
by the file name. Ex.   rustc hello_world.rs
Now run the compiled file with the .\ command.
Ex.   .\hello_world
This is how you compile and run a rust program.

Cargo is the main build system within rust, as well as a package manager. Because of cargo, alot of work
is actually simplified. Some of the features that Cargo look after in Rust
include:
- Development or building of user code, similar to leiningen or Boot for Clojure
- giving access to additional libraries or dependencies that a Rust progam needs by downloading them
- building of the above mentioned libaries or dependencies.

Creating a cargo project can be done with the command cargo new <project_name> --bin.
Here new creates a new project with the specified name for its project. The --bin command at the end
means that this project is created with intention that the user plans to create a binary program.

To build a project simply running cargo build should suffice. This creates a target file where the user can go further into the path
and choose the executable file to run. Cargo run as explained below seems to be a more wiser option as it does the same as build with just one move
and is easier to use.

Use the command cargo run within the project directory to be able to just compile the project and run all executable files within
the directory.

Cargo check is another command that can help verify for the user that the user's source code is good and gives confirmation
without actually creating the executable files.

## About the standard library

#### _Strings in Rust_

```
Strings are compound data types. To create and assign a string to a variable, we must call forth the string the :: operator.

Ex. let str1 = String ::from("This is a string"); 
```

We can mutate the string above using a method within the standard library for String called push_str.
```
str.push_str1(" too!");
Ex. println!(str1)  // This is a string too! 
```

Strings and other data types can also be spliced using references (&)
```
let newStr = &str1[8..15];  // the splicing is similar to the format of clojure's range specificiation
                            // the first index is inclusive and mentions the start of splice
                            // the last index, 15 is exlcusive and marks the end of the splice
Ex. println!(newStr)    // a string
```

#### _Other Data structures_

> Vectors 
- A very basic and useful collection, similar to the many vectors in various other languages. 
```
let vec = vec![1, 2, 3, 4];     // assigning to the variable vec a result of th the creation of a vector with 
for x in vec.iter() {           // help of the macro vector
    println!("Vec contains {}", x);
}
```
> HashMaps

- Hash maps can be created using the HashMap constructor.
- We can do this by including the following command at the very top of the file.
```
use std::collections::HashMap;
```

- A variable will refer to a HasMap intsance, user can then insert keys and values as long as they are within the 
 scope of the definition and declaration of the HashMap. Of course to be able to do this, the namespace must be included so the compiler understands what HashMap means.

```
let mut contacts = HashMap::new();
contacts.insert("James", "123-4567");
```


## About open source library

> _Describe at least one contribution by the open source
community written in the language._

# Analysis of the language

> _Organize your report according to the project description
document_.


