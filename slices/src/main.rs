fn titulo(mensaje: &String) -> String {
    let primera_letra = mensaje[0..1].to_uppercase();
    primera_letra.push_str( mensaje[1..] );

    primera_letra.to_string()
}

fn main() {
    // Slices -> Heap
    // Arrays -> Stack

    let mensaje = String::from("Hola mundo, desde el curso de Rust!");

    // let hola = &mensaje[0..4]; // [start..end]
    let hola = &mensaje[..4];
    let resto_mensaje = &mensaje[4..];
    let mensaje_completo = &mensaje[..];

    println!("El mensaje es: {}", mensaje);
    println!("El slice es: {}", hola);
    println!("El slice es: {}", resto_mensaje);
    println!("El slice es: {}", mensaje_completo);

    let titulo = titulo(&"hola mundo".to_string());
    println!("{}", titulo);

}
