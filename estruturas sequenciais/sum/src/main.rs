//Faça um Programa que peça dois números e imprima a soma.
use std::io;

fn sum(n1: &i32, n2: &i32) -> i32{
    *n1+*n2
}

fn main() {
    let mut n1:String = String::new();
    let mut n2:String = String::new();
    let msg:&str = "Entrada inválida";
    let msg2:&str = "NAN";

    println!("Digite o primeiro termo da soma: ");
    io::stdin().read_line(&mut n1).expect(msg);
    println!("Digite o segundo termo da soma: ");
    io::stdin().read_line(&mut n2).expect(msg);

    let n1:i32 = n1.trim().parse().expect(msg2);
    let n2:i32 = n2.trim().parse().expect(msg2);
    
    let sum:i32 = sum(&n1, &n2);
    println!("{}+{}={}", n1, n2, sum)
}
