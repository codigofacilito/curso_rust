struct Rectangulo {
    ancho: u32,
    alto: u32
}

fn area(rectangulo: &Rectangulo) -> u32 {
    rectangulo.ancho * rectangulo.alto
}

fn main() {
    // Ownership

    /*
    * Cada valor en Rust tiene si propio ownsership.
    * Solo puede existir un ownsership a la vez.
    * Si un ownsership sale del alcance, el valor se descartará.      
    */

    let rectangulo = Rectangulo { ancho:10, alto:20 };
    // Los argumentos son pasados mediante prestamos -> default
    // Los argumentos sean pasados por referencias
    let resultado = area(&rectangulo); 

    // Los objetos que se almacenan en HEAP
    let nuevo_rectangulo = rectangulo; // Movimiento de Ownership

    let x = 10; // Stack
    let y = x;

    println!("{}", x);
    println!("{}", y);

    println!("El área de rectangulo es: {}", resultado);
    println!("El ancho y alto del rectangulo es: {} - {}", nuevo_rectangulo.ancho, nuevo_rectangulo.alto);
}
