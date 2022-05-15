use kitchen::{infrastructure,scope};

fn main() {
    let funcionario = infrastructure::Employee::new(
        String::from("Adriano"),
        String::from(" Heller Mylla"),
        String::from("devops.heller@gmail.com"));

    let t = scope::Establishment::new("Grupo Taj","12998009000112","Avenida Batel 1167","4198770000",57);

    println!("{:#?}",t);

    println!("{:#?}",funcionario);
}