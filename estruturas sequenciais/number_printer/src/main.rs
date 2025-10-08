//Faça um Programa que peça um número e então mostre a mensagem O número informado foi [número].

use std::io;

fn main() {
    let mut input:String = String::new();
    println!("Digite um número: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Erro de entrada");

    let number:i32 = input.trim().parse().expect("NAN");
    println!("O número informado foi {}", number);
}
