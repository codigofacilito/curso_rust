#[derive(Debug)]
struct User {
    username: String,
    password: String,
}

impl User {
    fn saluda(& self) {
        println!("Hola, soy el usuario {}", self.username);
    }

    fn change_password(&mut self, new_password: String) {
        self.password = new_password;
    }
}

fn main() {
    
    let mut usuario1 = User{
        username: String::from("usuario1"),
        password: String::from("Password"),
    };

    usuario1.saluda();
    usuario1.change_password("Nuevo Password".to_string());

    println!("El usuario es: {:?}", usuario1); // {:?}

}
