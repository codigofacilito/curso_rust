fn main() {
            //   0     1     2
    let tupla = (10, false, 5.5);
    println!("El valor de la tupla es: {:?}", tupla);

    let mut tupla: (i32, bool, f64, i32) = (10, false, 5.5, 99); 
    println!("El valor de la tupla es: {:?}", tupla);

    let primer_elemento = tupla.0;
    let ultimo_elemento = tupla.3;

    tupla.1 = true;

    println!("El elemento es: {}", primer_elemento);
    println!("El elemento es: {}", ultimo_elemento);

    println!("El valor de la tupla es: {:?}", tupla);

}
