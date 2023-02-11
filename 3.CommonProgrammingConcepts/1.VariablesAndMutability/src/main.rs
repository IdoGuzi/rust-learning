const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("There are {THREE_HOURS_IN_SECONDS} seconds in 3 hours");

    //mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //static scoping
    let y = 7;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");


    //shadowing
    let str = "I'm a string";
    println!("str is a string: {str}");
    let str = 5;
    println!("str is no longer a string: {str}");
}
