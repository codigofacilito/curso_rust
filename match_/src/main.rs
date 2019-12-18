fn main() {
    
    for numero in 1..31 {

        match (numero % 3, numero % 5) {
            (0, 0) => println!("Fizz buzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", numero)
        }
    }
}
