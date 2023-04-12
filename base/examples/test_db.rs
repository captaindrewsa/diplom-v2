use base::dataStructures::{
    DataBaseControl::DatabaseGlobal,
    DatabaseControlInterface::{DatabaseClientInterface, DatabaseGlobalInterface},
};

fn main() {
    let mut a = DatabaseGlobal::check_and_get_of_connection().unwrap();
    println!(
        "{}",
        std::mem::size_of_val(&a.find_enzyme_by_entry("1.1.1.1").unwrap().to_string())
    );
}
