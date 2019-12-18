fn main() {
    
    let mensaje = Some("Hola mundo.");

    match mensaje {
        Some("Hola mundo") => println!("El mensaje es: Hola mundo"),
        Some("Adiós") => println!("El mensaje es: Adiós"),
        Some(_) => println!("Es otro mensaje"),
        None => println!("No existe valor alguno")
    }
    
}
