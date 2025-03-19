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
}
