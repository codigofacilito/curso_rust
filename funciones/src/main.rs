fn suma(numero_uno: i32, numero_dos: i32) -> i32 {
    numero_uno + numero_dos
}

fn factorial(numero: u32) -> u32 {
    if numero == 1 {
        numero
    }else{
        factorial(numero - 1) * numero
    }
}

fn main() {
    let resultado = factorial(5);

    println!("El factorial es: {}", resultado);

}
