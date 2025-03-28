// use std::io;

// fn main() {
//     // Sobre pattern match

//     let numero = 654654;

//     match numero {
//         1 => println!("O número é 1"),
//         2 => println!("O número é 2"),
//         3 | 4 => println!("O número é 3 ou 4"),
//         5..10 => println!("O número está entre 5 e 10"),
//         _ => println!("Não sei qual o número")
//     }

//     /*
//     Alguns outputs

//     juanvieira@juanvieira:~/local/rust/pattern_match$ cargo run
//    Compiling pattern_match v0.1.0 (/home/juanvieira/local/rust/pattern_match)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.35s
//      Running `target/debug/pattern_match`
//     O número é 2
//     juanvieira@juanvieira:~/local/rust/pattern_match$ cargo run
//     Compiling pattern_match v0.1.0 (/home/juanvieira/local/rust/pattern_match)
//         Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
//         Running `target/debug/pattern_match`
//     O número está entre 5 e 10
//     juanvieira@juanvieira:~/local/rust/pattern_match$ cargo run
//     Compiling pattern_match v0.1.0 (/home/juanvieira/local/rust/pattern_match)
//         Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.36s
//         Running `target/debug/pattern_match`
//     Não sei qual o número
//     */

//     let nome = "Lucas";

//     match nome {
//         "Lukas" => println!("O nome correto é Lucas, com 'C'."),
//         "Lucas" => println!("Correto!"),
//         _ => println!("Nenhuma entrada compatível")
//     }

//     // Input de dados no match

//     println!("---");

//     let mut msg = String::new();
//     println!("Digite alguma coisa aqui: \n\n");

//     match io::stdin().read_line(&mut msg){
//         Ok(_) => println!("Sucesso! Você digitou: {}", msg),
//         Err(e) => println!("Erro: {}", e)
//     };

//     /*
//     juanvieira@juanvieira:~/local/rust/pattern_match$ cargo run
//     Compiling pattern_match v0.1.0 (/home/juanvieira/local/rust/pattern_match)
//         Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.39s
//         Running `target/debug/pattern_match`
//     ---
//     Digite alguma coisa aqui: 


//     alguma coisa aqui
//     Sucesso! Você digitou: alguma coisa aqui
//     */

    
// }

struct Produto {
    nome: String,
    preco: f32,
}
 
trait Desconto {
    fn aplicar_desconto(&mut self, porcentagem: f32);
}
 
impl Desconto for Produto {
    fn aplicar_desconto(&mut self, porcentagem: f32) {
        self.preco -= self.preco * (porcentagem / 100.0);
    }
}
 
fn main() {
    let mut produto1 = Produto {
        nome: String::from("Notebook"),
        preco: 2000.0,
    };
    let mut produto2 = Produto {
        nome: String::from("Smartphone"),
        preco: 1500.0,
    };
 
    produto1.aplicar_desconto(10.0);
    produto2.aplicar_desconto(5.0);
 
    println!("O preço do {} é: R$ {:.2}", produto1.nome, produto1.preco);
    println!("O preço do {} é: R$ {:.2}", produto2.nome, produto2.preco);
}