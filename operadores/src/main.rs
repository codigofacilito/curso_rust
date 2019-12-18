fn main() {
    
    let numero_uno: i32 = 10;
    let numero_dos: i32 = 200;

    // + - / * %
    let resultado: i32 = numero_uno * numero_dos;

    // > < >= <= == !=
    let _resultado: bool = resultado != 2000;

    // && ||
    let resultado: bool = 20 + 10 > 100 && true;

    println!("El resultado es: {}", resultado);
}
