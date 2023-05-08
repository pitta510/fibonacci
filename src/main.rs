use std::io;
fn main() {
    println!("digite quantos números da sequencia de fibonacci você quer ver:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha!!!");

    let number: i32 = input.trim().parse().expect("msg");

    let mut ant = 0;
    let mut current: i32 = 1;
    println!("{}", ant);
    println!("{}", current);
    for _n in ant..number - 2 {
        let next: i32 = ant + current;
        ant = current;
        current = next;
        println!("{}", current);
    }
}
