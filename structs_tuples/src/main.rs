#[derive(Debug)]
struct Color(u32, u32, u32); // RGB

fn main() {

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    let mut custome_color = Color(187, 62, 104);
    custome_color.1 = custome_color.1 + 10; // 72

    println!("El color es: {:?}", black);
    println!("El color es: {:?}", white);
    println!("El color es: {:?}", custome_color);

}
