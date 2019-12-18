fn main() {
    // Fizz Buzz    

    for numero in 1..101 {
        
        if numero % 3 == 0 && numero % 5 == 0 {
            println!("Fizz Buzz");
        } else if numero % 3 == 0 {
            println!("Fizz")
        } else if numero % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", numero);
        }
    }

}
