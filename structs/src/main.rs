#[derive(Debug)]
struct Metadata {
    fileid: String,
    filename: String,
    description: String,
    createdat: String,
    datasource: String
}

struct Status (i32, String, bool);

fn metadata_print(name: &Metadata) {
    println!("{}", name.filename)
}

struct User {
    username: String,
    email: String,
    ativo: bool,
    genero: String
}

impl User {
    fn nome_do_usuario(&self) {
        println!("Nome do usuário: {}", self.username)
    }

    fn se_usuario_esta_ativo(&self) {
        if self.ativo == true {
            println!("Usuário: ativo.")
        } else {
            println!("Usuário: inativo.")
        }
    }

    fn provedor(&self) {
        if self.email.contains("example") {
            println!("O provedor é 'example'.")
        } else {
            println!("Este é o e-mail: {}", self.email)
        }
    }

    fn one_hot_encoding_fake(&self){
        if self.genero == "M" {
            println!("0")
        } else if self.genero == "F" {
            println!("1")
        } else {
            println!("O gênero específicado pela pessoa foi: {}", self.genero)
        }
    }
}

fn main() {
    let my_data = Metadata{
        fileid: String::from("0ais5u7dh987394Hio5"),
        filename: String::from("nlp_features.parquet"),
        description: String::from("Features that can be used for NLP tasks."),
        createdat: String::from("14/02/2024"),
        datasource: String::from("Yahoo Finance API")
    };

    println!("File ID: {}", my_data.filename);
    println!("File Name: {}", my_data.fileid);
    println!("Description: {}", my_data.description);
    println!("Created At: {}", my_data.createdat);
    println!("Data Source: {}", my_data.datasource);

    println!("------------------------");

    metadata_print(&my_data);
    metadata_print(&my_data);
    println!("------------------------");

    // tuple structs
    
    let new_file = Status(1, String::from("control.csv"), true);

    println!("{}", new_file.0);
    println!("{}", new_file.1);
    println!("{}", new_file.2);

    // sobre impl
    println!("---");
    println!("Estudando sobre impl:");

    let pessoa1 = User{
        username: String::from("joao"),
        email: String::from("joao@example.com"),
        ativo: true,
        genero: String::from("M")
    };

    pessoa1.nome_do_usuario();
    pessoa1.se_usuario_esta_ativo();
    pessoa1.provedor();
    pessoa1.one_hot_encoding_fake();
    
    println!("---");

    let pessoa2 = User{
        username: String::from("lucas"),
        email: String::from("lucas@gmail.com"),
        ativo: true,
        genero: String::from("M")
    };

    pessoa2.nome_do_usuario();
    pessoa2.se_usuario_esta_ativo();
    pessoa2.provedor();
    pessoa2.one_hot_encoding_fake();
}
