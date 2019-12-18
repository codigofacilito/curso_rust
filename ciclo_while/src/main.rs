fn main() {
    
    // 123 -> 3 = 12
    // 12345 -> 5
    // 123456789 -> 9

    let mut numero = 123456789;
    let mut contador = 0;

    while numero > 0 {
        numero = numero / 10;
        contador += 1;
    }

    println!("La cantidad de dígitos son: {}", contador);
}
