use std::io::Read;

pub fn run(file: Option<String>) {
    // read the YAML content from stdin or from a file
    let yaml_content = match file {
        Some(file_path) => std::fs::read_to_string(file_path).expect("Failed to read file"),
        None => {
            let mut input = String::new();
            std::io::stdin()
                .read_to_string(&mut input)
                .expect("Failed to read from stdin");
            input
        }
    };

    // parse the YAML content
    let yaml_value: serde_yaml::Value =
        serde_yaml::from_str(&yaml_content).expect("Failed to parse YAML content");

    // convert the YAML value to JSON
    let json_content =
        serde_json::to_string_pretty(&yaml_value).expect("Failed to convert to JSON");

    // print the JSON content to stdout
    println!("{}", json_content);
}
