fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let x2 = 5;

    let x2 = x2 + 1;

    {
        let x2 = x2 * 2;
        println!("The value of x in the inner scope is: {x2}");
    }

    println!("The value of x is: {x2}");
}
