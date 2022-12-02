use std::fs::File;
use std::io::Read;

pub fn open_file_into_string(path: &str) -> String {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("no file"),
    };

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("failed to read");

    file_contents
}

pub fn split_by_newline(input: &str) -> Vec<String> {
    input.split('\n')
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>()
}

pub fn string_into_vec_of_uint_radix(input: String, radix: u32) -> Vec<u64> {
    input.split('\n')
                      .map(|s| s.to_string())
                      .filter(|s| !s.is_empty())
                      .map(|s| u64::from_str_radix(&s, radix).unwrap())
                      .collect::<Vec<u64>>()
}

pub fn string_into_vec_of_uint(input: String) -> Vec<u64> {
    string_into_vec_of_uint_radix(input, 10)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
