fn main() {
    // let <nombre de la variable> = <El valor>
    // let <nombre de la variable>: <T> = <El valor>
    // let mut <nombre de la variable>: <T> = <El valor>
    
    const VALOR: i32 = 10;

    let mut numero_uno = 15;
    let numero_dos: i32 = 10;

    numero_uno = 100;

    let resultado = numero_uno + numero_dos + VALOR;

    println!("El resultado es ({} + {} + {}): {}", numero_uno, numero_dos, VALOR, resultado);
}
