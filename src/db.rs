fn get_file_create_if_not_exists(file_path: &str) -> std::fs::File {
    match std::fs::File::open(file_path) {
        Ok(file) => file,
        Err(_) => match std::fs::File::create(file_path) {
            Ok(file) => file,
            Err(_) => panic!("Could not create file"),
        },
    }
}

#[test]
fn get_file_handler() {
    let file = get_file_create_if_not_exists("test.csv");
    assert!(file.metadata().is_ok());
}
