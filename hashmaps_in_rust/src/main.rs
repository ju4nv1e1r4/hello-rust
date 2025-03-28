use std::collections::HashMap;

fn main() {
    let mut my_first_hashmap = HashMap::new();

    println!("### ML Models");

    println!("---");    
    my_first_hashmap.insert("RandomForestClassifier", 91);
    my_first_hashmap.insert("KNN", 89);
    my_first_hashmap.insert("LogisticRegressor", 85);
    my_first_hashmap.insert("XGBoostClassifier", 95);
    my_first_hashmap.insert("LightGBM", 94);

    println!("{:#?}", my_first_hashmap);
    println!("---");

    println!("Iterando sobre os valores");
    for acc in my_first_hashmap.values(){
        println!("Accuracy: {acc}%")
    }

    println!("---");

    println!("Iterando sobre as chaves");
    for model in my_first_hashmap.keys(){
        println!("Model: {model}")
    }
    println!("---");

    match my_first_hashmap.get("KNN"){
        Some(algo) => println!("A acurácia do modelo usando KNN foi {algo}%."),
        None => println!("O algoritmo não foi utilizado ou sua pontuação não foi registrada.")
    }
    println!("---");
    let model_to_remove = "LogisticRegressor";
    my_first_hashmap.remove(model_to_remove);

    let verify_remove = my_first_hashmap.contains_key(model_to_remove);
    if verify_remove == false {
        println!("Model is removed from registry.")
    } else {
        println!("Remove status: {verify_remove}")
    }
    println!("---");
    println!("{:#?}", my_first_hashmap);
}
