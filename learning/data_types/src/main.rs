fn main() {
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';
    println!("{decimal}");
    println!("{hex}");
    println!("{octal}");
    println!("{binary}");
    println!("{byte}");

    let num1: i8 = 127;
    // let _num2: i8 = num1 + 1;

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
}
