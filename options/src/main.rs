/*
enum Option<T>{
    Some(T), -> El valor
    None -> La ausencía de algún valor
}
*/
#[derive(Debug)]
struct User {
    username: String,
    password: String,
    email: String,
    edad: Option<u32>
}

fn main() {
    
   let usuario1 = User{
        username: String::from("eduardo"),
        password: String::from("password"),
        email: String::from("eduardo@codigofacilito.com"),
        edad: None //Some(26)
   };

   println!("El usuario es: {:?}", usuario1);

   // let edad = usuario1.edad.unwrap(); // match

   match usuario1.edad {
       Some(edad) => println!("Su edad es: {}", edad),
       None => { },
   }

}   
