use std::collections::HashMap;

pub fn set_command(store: &mut HashMap<String, String>, key: String, value: String) {
    store.insert(key, value);
    println!("Success");
}

pub fn get_command(store: &HashMap<String, String>, key: &str) {
    match store.get(key) {
        Some(value) => println!("{}", value),
        None => println!("(nil)"),
    }
}

pub fn del_command(store: &mut HashMap<String, String>, key: &str) {
    if store.remove(key).is_some() {
        println!("Success");
    } else {
        println!("Key not found");
    }
}
