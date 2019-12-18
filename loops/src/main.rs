fn main() {
    
    // loop

    let mut contador = 0;

    loop {
        contador += 1;

        println!("Contador: {}", contador);

        if contador >= 10 {
            break;
        }
        
    }

}
