fn main() {
    let tup = ("test", 1);
    let (chan, subcount) = tup;

    println!("{}", tup.1);

    let byte = [0; 8];
    for b in byte.iter() {
        println!("{}", b);
    }

    let condition = true;
    let a = if condition { 6 } else { 8 };
}
