fn main() {
    {
        let primeiro_nome = String::from("João");
        println!("{primeiro_nome}");
        println!("{}", primeiro_nome.replace("João", "Lucas"));
    }

    {
        let ms = String::from("linha1\nlinha2\nlinha3");
        for i in ms.lines(){
            println!("(_{}_)", i);
        }
    }

    {
        let ms = String::from("caminho+para+casa");
        let token: Vec<&str> = ms.split("+").collect();
        println!("{:?}", token);
    }

    {
        let ms = String::from("caminho        para        casa");
        println!("{}", ms.trim());
    }

    {
        let ms = String::from("Rust é uma linguagem muito eficiente");
        match ms.chars().nth(11){
            Some(c) => println!("Ok ------> {}", c),
            None => println!("Erro")
        }
    }
}
