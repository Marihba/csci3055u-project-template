fn main() {
    let x = 50;
    let mut a = 12;
    let nums = [1, 2, 3, 4, 5];

    // while loops
    while a!= 0 {
        println!("{}", a); //here the {} is rust's way of saying print with format
        a-=1;
    }

    // single if/else statements
    println!("Is {} smaller than 10?", x);
    if x < 10 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }

// multiple if/else branchings
    let z = 4;
    println!("What is mod of {} ?", z);
    if a % z == 0 {
        println!("A is divisible by 4");
    } else if a % 3 == 0 {
        println!("A is divisible by 3");
    } else if a % 2 == 0 {
        println!("A is divisible by 2");
    } else {
        println!("A is not divisible by 4, 3 and 2!");
    }

    let result = sum(10, 5);
    println!("The value of this function {}", result);
}

// the -> indicates that the function will return a result of type i32, integer 32 bits
fn sum(x:i32, y:i32) -> i32 {
    let mut fin_val = x + y;
    fin_val = fin_val * 123;
    return fin_val;
}
