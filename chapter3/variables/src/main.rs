fn main() {
    // Understanding the mut keyword
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Understanding constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Number of seconds in three hours: {THREE_HOURS_IN_SECONDS}");

    // Understanding shadowing
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");
}
