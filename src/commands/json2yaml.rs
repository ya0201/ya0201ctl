use std::io::Read;

pub fn run(file: Option<String>) {
    // read the JSON content from stdin or from a file
    let json_content = match file {
        Some(file_path) => std::fs::read_to_string(file_path).expect("Failed to read file"),
        None => {
            let mut input = String::new();
            std::io::stdin()
                .read_to_string(&mut input)
                .expect("Failed to read from stdin");
            input
        }
    };

    // parse the JSON content
    let json_value: serde_json::Value =
        serde_json::from_str(&json_content).expect("Failed to parse JSON content");

    // convert the JSON value to YAML
    let yaml_content = serde_yaml::to_string(&json_value).expect("Failed to convert to YAML");

    // print the YAML content to stdout
    println!("{}", yaml_content);
}
