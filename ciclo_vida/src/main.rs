fn main() { 

    let mut mensaje = String::from("Hola, soy una variable para prestamo.");

    {

        let prestamo = &mensaje; // Prestamos -> Se mueve

        mensaje = String::from("Cambio de valor");

        println!("{}", prestamo); // Freezing -> mensaje

    }

    println!("{}", mensaje);
}
