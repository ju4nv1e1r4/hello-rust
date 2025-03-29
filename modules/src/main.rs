mod method;

fn main() {
    method::my_test_method();

    let user = method::Pessoa{
        nome: String::from("Lucas"),
        idade: 1
    };

    println!("{:?}", user);

    user.qual_o_nome();
    user.qual_a_idade();
}
