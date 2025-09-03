mod cli;
mod model;
mod store;

fn main() {
    println!("Hello");

    println!("{:?}", model::DataFile {
        schema_version: 1,
        habits: Vec::new(),
        completions: Vec::new(),
    });
}