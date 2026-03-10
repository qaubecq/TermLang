use std::collections::HashMap;


#[derive(Debug)]
struct CodeLine {
    pub code: String,  // The line of code
    pub depth: u8,   // How deep is the line in closures
    pub starts_closure: bool   // Wether this line is a function/procedure signature, if or while
}


pub fn kernel(code: String, formated: bool) -> String {
    let mut lines: Vec<CodeLine> = to_code_lines(code);
    println!("{:?}", lines);
    define(&mut lines);
    println!("{:?}", lines);


    // Remove lines that start with # and get the terminal size

    // Parse to remove excess spaces and closures, panic on invalid syntax (check if closure starts lines start with proc, if or while)
    // TODO

    return "".to_string();
}


fn to_code_lines(c: String) -> Vec<CodeLine> {
    // Remove comments
    let mut code = String::new();
    let mut in_comment = false;
    for i in 0..c.len() {
        if c.chars().nth(i).unwrap() == '\n' {
            in_comment = false;
        }
        else if i < c.len()-1 && c.chars().nth(i).unwrap() == '/' && c.chars().nth(i+1).unwrap() == '/' {
            in_comment = true;
        }
        if !in_comment {
            code.push(c.chars().nth(i).unwrap());
        }
    }
    
    // Remove \n and \t
    code = code.replace('\n', "").replace('\t', "");
    // Split in lines (; as a delimiter)
    let text_lines = code.split(';').collect::<Vec<&str>>();

    // Create CodeLines
    let mut lines: Vec<CodeLine> = Vec::new();
    let mut depth: u8 = 0;
    for line in text_lines {
        let mut buffer: String = String::new();
        for char in line.chars() {
            if char == '{' {
                // Remove leading and trailing spaces
                let str = buffer.trim().to_string();
                lines.push(CodeLine { code: str, depth: depth, starts_closure: true}); // Empty lines can be pushed here, this allows the rest of kerneler to know that a new closure was opened, this will be removed on the final parse
                buffer.clear();
                depth += 1;
            } else if char == '}' {
                if depth==0 {
                    panic!("Kerneler Error : Closing a closure that was never opened");
                }
                depth -= 1;
            } else {
                buffer.push(char);
            }
        }
        let str = buffer.trim().to_string();
        if !str.is_empty() {
            lines.push(CodeLine { code: str, depth: depth, starts_closure: false });
        }
    }

    return lines;
}


struct Stack {
    pub activated: bool,
    start: (u8, u8),
    end: (u8, u8),
    current: (u8, u8, u8)
}
impl Stack {
    pub fn new() -> Self {
        return Stack { activated: false, start: (0, 0), end: (0, 0), current: (0, 0, 0)};
    }

    pub fn activate(&mut self, start: (u8, u8), end: (u8, u8)) {
        // Check if start and end are valid
        if start.0 > end.0 || start.1 > end.1 {
            panic!("Kerneler Error : Invalid stack dimensions");
        }
        if self.activated {
            panic!("Kerneler Error : Stack activated twice");
        }
        self.activated = true;
        self.start = start;
        self.end = end;
        self.current = (start.0, start.1, 0);
    }

    pub fn get(&mut self) -> (u8, u8, u8) {
        if self.current.2 == 3 {
            panic!("Kerneler Error : Stack Overflow");
        }
        // Returns current and increments current
        let ret = self.current;
        if self.current.2 < 2 {
            self.current.2 += 1;
        } else if self.current.0 < self.end.0 {
            self.current.2 = 0;
            self.current.0 += 1;
        } else if self.current.1 < self.end.1 {
            self.current.2 = 0;
            self.current.0 = 0;
            self.current.1 += 1;
        } else {
            self.current.2 = 3;
        }

        return ret;
    }

    pub fn get_used(&self) -> (u32, u32) {
        return (((self.current.0-self.start.0) as u32)*3 + ((self.current.1-self.start.1) as u32)*((self.end.0-self.start.0 + 1) as u32)*3 + self.current.2 as u32, 3*((self.end.0-self.start.0) as u32 + 1)*((self.end.1-self.start.1) as u32 + 1));
    }
}

fn define(lines: &mut Vec<CodeLine>) {
    // Create stack struct (inactive until #struct is found)
    let mut stack: Stack = Stack::new();

    // Create Vec<HashMap> (one dict of defines per depth), when a variable is found it will be replaced by the closest occurence of that variable in the Vec<HashMap>
    let mut maps: Vec<HashMap<String, String>> = Vec::new();
    // Push global map
    maps.push(HashMap::new());
    let mut previous_depth = 0;
    for line in lines {
        // If we went one level deeper, add a HashMap to maps, if we went one level above, remove the top hash map
        if line.depth > previous_depth {
            maps.push(HashMap::new());
        } else if line.depth < previous_depth {
            maps.pop();
        }
        previous_depth = line.depth;

        // If the line is #stack x1 y1 x2 y2, activate the stack
        if line.code.starts_with("#stack") {
            let [x1, y1, x2, y2]: [u8; 4] = line.code.split_whitespace().skip(1).map(|v| v.parse().expect("Kerneler Error : Invalid #stack command")).collect::<Vec<_>>().try_into().expect("Kerneler Error : Invalid #stack command");
            stack.activate((x1, y1), (x2, y2));
            println!("Kerneler Log : Stack activated with size {}", stack.get_used().1);
        }

        // If the line startswith "define", add an entry to the top hashmap
        if line.code.starts_with("define") {
            let args: Vec<&str> = line.code.split_whitespace().skip(1).collect();
            if args.len() == 1 {
                if !stack.activated {
                    panic!("Kerneler Error : define value unspecified with stack not activated\n | {}", line.code);
                }
                let value = stack.get();
                maps.last_mut().unwrap().insert(args[0].to_string(), format!("[{},{},{}]", value.0, value.1, value.2).to_string());
            }
            else if args.len() == 2 {
                maps.last_mut().unwrap().insert(args[0].to_string(), args[1].to_string());
            } else {
                panic!("Kerneler Error : Invalid number of arguments for define\n | {}", line.code);
            }
            continue;
        }

        // Iterate backwards on the string to replace all known defined names
        let mut current_word = String::new();
        for i in (0..line.code.len()).rev() {
            let c = line.code.chars().nth(i).unwrap();
            if !c.is_alphanumeric() {
                if !current_word.is_empty() {
                    replace_word(&maps, &current_word, &mut line.code, i+1);
                    current_word.clear();
                }
            } else {
                current_word.insert(0, c);
            }
        }
        if !current_word.is_empty() {
            replace_word(&maps, &current_word, &mut line.code, 0);
        }
    }
}


fn replace_word(maps: &Vec<HashMap<String, String>>, word: &String, line: &mut String, index: usize) {
    // Check if the word is in the maps
    for map in maps.iter().rev() {
        if map.contains_key(word) {
            let replacer = map.get(word).unwrap();
            let old_line = line.clone();
            line.replace_range(index..(index+word.len()), replacer);
            println!("Kerneler Log : Replaced occurence of {} that was previously defined with {} (<{}>  ==>  <{}>)", word, replacer, old_line, line);
            break;
        }
    }
}