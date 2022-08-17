fn main() {
    // {.. different scope..};
    let y = {
        let x = 3; // statement requires a semi-colon
        x + 1 
        // expression doesn't require semi-colon
        // this is what is returned from this scope
    };

    println!("The value y is {}", y);
}
