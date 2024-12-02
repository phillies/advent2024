// Convenience function which reads all lines from a file into a vector of strings
pub fn read_input_to_vector(filename: &str) -> Vec<String> {
    let input = std::fs::read_to_string(filename).expect("Could not read file!");
    input.lines().map(|s| s.to_string()).collect()
}
