###Rusk Programming Language Research Project

- Abhiram Sinnarajah
- Abhiram.sinnarajah@uoit.net

## About the language

> _Describe the language_
>
> - History
> - Some interesting features

## About the syntax

> _give some code snippet of the language_

*Let form*

```clojure
(let [x 10
      y 20]
  (+ x y))
```

## About the tools

Rust is an ahead-of-time compiled language. This means that source code represeneted by
the file_name.rs needs to be compiled in order for it to be able to run the code. The compiled code
is a binary ececutable. The machine runs this executable generated for the user. To compile a file, type rustc command followed
by the file name. Ex.   rustc hello_world.rs
Now run the compiled file with the .\ command.
Ex.   .\hello_world.exe
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

> _Give some examples of the functions and data structures
> offered by the standard library_.

## About open source library

> _Describe at least one contribution by the open source
community written in the language._

# Analysis of the language

> _Organize your report according to the project description
document_.


