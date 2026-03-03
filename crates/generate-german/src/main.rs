fn main() {
    let toml_str = include_str!("../mapping.toml");
    let mapping: toml::Value = toml::from_str(toml_str).unwrap();
    println!(
        "Loaded {} BO mappings",
        mapping["bo"].as_table().unwrap().len()
    );
    println!(
        "Loaded {} COM mappings",
        mapping["com"].as_table().unwrap().len()
    );
    println!(
        "Loaded {} enum mappings",
        mapping["enums"].as_table().unwrap().len()
    );
}
