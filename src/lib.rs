use std::{fs, error::Error};

pub fn read_file(file_path: String) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}

pub fn interpret(contents: String) {
    let mut cells: [u8; 30000] = [0; 30000];
    let mut i = 0;
    let mut chars_index = 0;
    let chars: Vec<char> = contents.chars().collect();

    loop {
        if chars_index == contents.len() {
            break;
        }
        let c = chars[chars_index];

        if c == '<' {
            i -= 1;
        }
        else if c == '>' {
            i += 1;
        }
        else if c == '-' {
            cells[i] -= 1;
        }
        else if c == '+' {
            cells[i] += 1;
            
        }
        else if c == '.' {
            print!("{}", String::from_utf8(vec!(cells[i])).expect("Unknown ASCII value."));
        }
        else if c == ',' {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Unable to read line.");
            cells[i] = input.as_bytes()[0];
        }
        else if c == ']' && cells[i] != 0 {
            let mut loop_count = 1;
            while loop_count > 0 {
                chars_index -= 1;
                let current_char = chars[chars_index];
                if current_char == '[' {
                    loop_count -= 1;
                }
                else if current_char == ']' {
                    loop_count += 1;
                }
            }
        }

        chars_index += 1;
    }

}
