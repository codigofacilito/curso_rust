fn main() {
    
    let calificacion: i8 = 10;
    
    let mensaje = if calificacion == 10 {
        String::from("Felicitaciones aprobaste la materia. 🦀 🎉")
    } else {
        String::from("Necesitas estudiar más")
    };

    println!("{}", mensaje);

}
