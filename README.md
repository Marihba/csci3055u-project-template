# Rust Programming Language Research Project

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

## Some interesting open source contributions by Rust

> *Rust Python*

This is an intersting open source project that is basically an interpreter of the Python 3 shell in Rust!!

To be able to run such a program, user must first clone the following git 

```
git clone https://github.com/RustPython/RustPython
```

Once available, the user can simply cd into the project directive and start using the interpreter by entering the cargo run command. This is a clean implentation and zero compatability hacks. Documentations of the project is 



> *Firecracker*

An open source virtualization technology with a main purpose for building and manage secure, multi-tenant container and function-based services for serverless opeational models. The main component of Firecracker is the Linux Kernel Virtual Machine (KVM). Developed at AWS to accelerate the speed and effieciency of services such as AWS Lamdbda and AWS Fargate. 

To be able to run Firecracker, user must first clone the following git 

```
git clone https://github.com/firecracker-microvm/firecracker
```

Once available the user can cd into the project directory and do cargo run.

Security features of the Firecracker allow for safe multi-tenant computing, assuming a well balanced and configured Linux host OS is used. 

# Analysis of the language

### *Style of Rust Programming Language*

Rust is a functional programming language with some additional features similar to C++. Function are created with the help of the fn operator. Main is the first entry point for a program, similar to majority of other languages. Rust requires semi-colon at the end of a statement.

The style of this language is snake case, meaning everything must be represented with lower cases and if necessary underscores. Look at example below:

```
let my_var = 123;
```

Rust's data types are eithe primitve or compound. Primitive meaning these are the most basic and 'Simple' data types, similar to Clojure simple. Types are referenced with a colon, and have many options for say integers.

Example:
- i8 (8 bit signed integers)
- i32 (16 bit signed integers)
- i32 (32 bit signed integers)
- i64 (64 bit signed integers)

Similar for Floats also and also includes variations for unsigned integers as well.
### *Meta Programming*

Rust has many amazing macro features, some of the most common being println! to even using Macros for exception handling. For instance one specific Macro called Panic! is used as a worst case situation arises during the programs run. In this case, when a failure occurs the program will indicate user with a failure message, back track ("reroll"), cleans stack and quits the program; how awesome is that!

An example of this usage would be like follows:

```
let vec = vec![1, 2, 3, 4, 5];
v[50] // clearly this should cause an error
```

Running this code above in the main results in the follow response from the compiler. 

```
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 50', libcore/slice/mod.rs:2448:10
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```



### *Symbols and scopes in Rust*

Rust uses the Principle of Ownership for symbols.

An example of a scope would be a variable and we can use Let in order to initialize and declare such variables.

```
let x = 100;
```
Here x is a symbol that is bound to the value 100. The program also follows static scoping, meaning lexical scope. To explicitly state a scope, the use of curly braces help.

```
{let x = 12;
println!("x: {}", x);
}
println!("what was x again?", x);
```

The last line above will not work as it is outside the scope of where x was initially defined. Hence Rust would throw out an exception stating x is undefined.

### *Rust Programming Constructs*

Rust provides users access to the static and heap memory. Because of this, referencing and pointers are an important concept in Rust. Users are able to refer to the memory location of a scope for variables using the & similar to c++

```
&x = memory address of where data of x is stored in
```

This means that heap space is very important and costly, similar to other C++. Which means deallocation from users are a must. The great thing about Rust is that variable information on the heap are deallocated as soon as the variable goes out of scope. An amazing construct that Rust encorporates.

Rust also supports object oriented programming as it embodies structs and enums.
The struct follows similar implementation as the structs used in C++ shown below.

```
struct Course { // types are indicated after the variable names
  title: String,  
  code: String,
  Capacity: i32,  // as mentioned above, this type means integer of size 32, 
  average: f32    // likewise for the average, a float of type 32
};
```

Other than the way the types are defined for this struct, it is quite similar and elegant as the structs we have seen in other languages. 

Rust has its own operator known as match. This is an operator that helps a user compare one value against a series of preset patterns. Once a match is found, it will execute the true portion of the code. Below is an example run through of matching patterns for a given string.

```
  let number = 3;
  match number {
    1 => println!("It's one!"),
    2 => println!("It's two!"),
    3 => println!("It's three!"),
    _ => (),    // here the _ means all other alternatives specified after the above cases.
}
```

Operator such as these make this language very robust and widespread.

### *Additional information on Rust*

List of Pros
- very simple to use
- statically typed
- powerful memory allocation safety
- concurrency safety
- gives ability for mutable data
-

Cons of Rust
- Referencing can be confusing
- Function returns can be ambiguous at times, careful with statements!
- Having mutable data possible can cause issues if data changes unexpectedly
- Heap is useful, but accessing heap can be a little slow at times


