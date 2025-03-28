struct Pessoa {
    nome: String,
    idade: i32
}

trait Voz {
    fn falar(&self);

    fn tem_voz(&self) -> bool;
}

impl Voz for Pessoa {
    fn falar(&self) {
        println!("Olá, meu nome é {}", self.nome);
    }

    fn tem_voz(&self) -> bool {
        if self.idade > 1 {
            return true;
        } else {
            return false;
        }
    }
}

fn main() {
    let usuario = Pessoa {
        nome: String::from("Juan"),
        idade: 29
    };

    println!("{} pode falar?\n{}", usuario.nome, usuario.tem_voz());
    println!("{:?} e eu tenho {} anos de idade", usuario.falar(), usuario.idade);
}
