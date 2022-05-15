use kitchen::{infrastructure,scope};

fn main() {
    let funcionario = infrastructure::Employer::new(
        String::from("Adriano"),
        String::from(" Heller Mylla"),
        String::from("devops.heller@gmail.com"));

    let t = scope::

    println!("{:#?}",funcionario);
}