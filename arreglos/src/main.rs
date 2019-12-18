fn main() {
                // 0  1  2  3  4
    let numeros = [1, 2, 3, 4, 5]; // size -> 5
    println!("El valor del arreglo es: {:?}", numeros);

    let mut numeros: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("El valor del arreglo es: {:?}", numeros);

    let valores = [1; 10];
    println!("El valor del arreglo es: {:?}", valores);

    let primer_elemento = numeros[0];
    let ultimo_elemento = numeros[numeros.len() - 1];

    numeros[2] = 30;

    println!("El elemento es: {}", primer_elemento);
    println!("El elemento es: {}", ultimo_elemento);

    println!("El valor del arreglo es: {:?}", numeros);
}
