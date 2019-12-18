fn main() {
    
    let valor: i32 = 10; // Inmutable

    println!("El valor de la variable es: {}", valor);

    let valor = 20; // Shadowing

    println!("El valor de la variable es: {}", valor);

    let valor = false;

    println!("El valor de la variable es: {}", valor);

}
