#[derive(Debug)]
pub struct Pessoa {
    pub nome: String,
    pub idade: i32
}

impl Pessoa {
    pub fn qual_o_nome(&self){
        println!("{}", self.nome);
    }

    pub fn qual_a_idade(&self){
        println!("{}", self.idade);
    }
}

pub fn my_test_method() {
    println!("This is a method")
}