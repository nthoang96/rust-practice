fn main() {
    // ----- Ownership rules -----
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one onwer at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    {
        // s is not valid here, it's not yet declared
        let s = String::from("Hi"); // s is valid from this point forward
                                    // do stuff with s
    } // this scope is now over, and s is no longer valid

    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1; // Move (not shallow copy)

    println!("{}, world!", s1);
}
