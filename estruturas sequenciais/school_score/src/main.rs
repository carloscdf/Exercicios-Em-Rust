//Faça um Programa que peça as 4 notas bimestrais e mostre a média.

use std::io;

fn calculate_score(value:f64)->f64{
    value/4.0
}

fn main() {
    let mut input:String = String::new();
    let mut value:f64 = 0.0;
    let input_error_msg:&str = "Entrada inválida";
    let parse_error_msg:&str = "NAN";

    println!("=== CALCULAR A MÉDIA ===\n");

    println!("Digite o primeiro valor: ");
    io::stdin().read_line(&mut input).expect(input_error_msg);
    value += input.trim().parse::<f64>().expect(parse_error_msg); //converte a string em inteiros de 32bits
    input.clear(); //limpa o valor de entrada digitado previamente

    println!("Digite o segundo valor: ");
    io::stdin().read_line(&mut input).expect(input_error_msg);
    value += input.trim().parse::<f64>().expect(parse_error_msg);
    input.clear();

    println!("Digite o terceiro valor: ");
    io::stdin().read_line(&mut input).expect(input_error_msg);
    value += input.trim().parse::<f64>().expect(parse_error_msg);
    input.clear();

    println!("Digite o quarto valor: ");
    io::stdin().read_line(&mut input).expect(input_error_msg);
    value += input.trim().parse::<f64>().expect(parse_error_msg);
    input.clear();

    let score:f64 = calculate_score(value);

    println!("\n=== A média do aluno é:  {} ===", score);

}
