use kitchen::infrastructure;

fn main() {
    let funcionario = infrastructure::Employer::new(
        String::from("Adriano"),
        String::from(" Heller Mylla"),
        String::from("devops.heller@gmail.com"));

    println!("{:#?}",funcionario);
}