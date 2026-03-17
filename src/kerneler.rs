use core::panic;
use std::collections::HashMap;

pub fn kernel(code: String) -> String {
    // Parse to remove excess spaces (remove all spaces around certain delimiters)
    // TODO

    let mut lines: Vec<CodeLine> = to_code_lines(code);
    println!("{:?}", lines);
    define(&mut lines);
    println!("{:?}", lines);
    reference(&mut lines);
    println!("{:?}", lines);
    arg_reference(&mut lines);
    println!("{:?}", lines);
    pointers(&mut lines);
    println!("{:?}", lines);
    

    return "".to_string();
}


#[derive(Debug)]
struct CodeLine {
    pub code: String,  // The line of code
    pub depth: u8,   // How deep is the line in closures
    pub starts_closure: bool   // Wether this line is a function/procedure signature, if or while
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
    let mut i = 0;
    while i < lines.len() {
        // If we went one level deeper, add a HashMap to maps, if we went one level above, remove the top hash map
        if lines[i].depth > previous_depth {
            maps.push(HashMap::new());
        } else if lines[i].depth < previous_depth {
            maps.pop();
        }
        previous_depth = lines[i].depth;

        // If the line is #stack x1 y1 x2 y2, activate the stack
        if lines[i].code.starts_with("#stack") {
            let [x1, y1, x2, y2]: [u8; 4] = lines[i].code.split_whitespace().skip(1).map(|v| v.parse().expect("Kerneler Error : Invalid #stack command")).collect::<Vec<_>>().try_into().expect("Kerneler Error : Invalid #stack command");
            stack.activate((x1, y1), (x2, y2));
            println!("Kerneler Log : Stack activated with size {}", stack.get_used().1);
            // Delete line
            lines.remove(i);
            continue;
        }

        // If the line startswith "define", add an entry to the top hashmap
        if lines[i].code.starts_with("define") {
            let args: Vec<&str> = lines[i].code.split_whitespace().skip(1).collect();
            if args.len() == 1 {
                if !stack.activated {
                    panic!("Kerneler Error : define value unspecified with stack not activated\n | {}", lines[i].code);
                }
                let value = stack.get();
                maps.last_mut().unwrap().insert(args[0].to_string(), format!("[{},{},{}]", value.0, value.1, value.2).to_string());
            }
            else if args.len() == 2 {
                maps.last_mut().unwrap().insert(args[0].to_string(), args[1].to_string());
            } else {
                panic!("Kerneler Error : Invalid number of arguments for define\n | {}", lines[i].code);
            }
            // Delete line
            lines.remove(i);
            continue;
        }

        // Iterate backwards on the string to replace all known defined names
        let mut current_word = String::new();
        for j in (0..lines[i].code.len()).rev() {
            let c = lines[i].code.chars().nth(j).unwrap();
            if !(c.is_alphanumeric()||c=='_') {
                if !current_word.is_empty() {
                    replace_word(&maps, &current_word, &mut lines[i].code, j+1);
                    current_word.clear();
                }
            } else {
                current_word.insert(0, c);
            }
        }
        if !current_word.is_empty() {
            replace_word(&maps, &current_word, &mut lines[i].code, 0);
        }
        i += 1;
    }


    // Remove unecessary closures (as they were only used for define scopes)
    let mut removing_closure_depths: Vec<u8> = Vec::new(); // The different depths at which an unecessary closure was opened and needs to be closed, each line will get shifted back by removing_closure_depth.len()
    let mut i = 0;
    while i < lines.len() {
        while !removing_closure_depths.is_empty() && lines[i].depth <= *removing_closure_depths.last().unwrap() {
            removing_closure_depths.pop();
        }
        if lines[i].code.is_empty() && lines[i].starts_closure {
            removing_closure_depths.push(lines[i].depth);
            lines.remove(i);
            continue;
        }
        if removing_closure_depths.is_empty() {
            i += 1;
            continue;
        }
        
        // Shift back by removing_closure_depths.len()
        lines[i].depth -= removing_closure_depths.len() as u8;
        i += 1;
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


fn reference(lines: &mut Vec<CodeLine>) {
    for line in lines {
        // Find the indices of &[ and corresponding ]
        let mut start_remove_indices: Vec<usize> = Vec::new();
        let mut end_remove_indices: Vec<usize> = Vec::new();
        let mut in_bracket = false;
        let mut depth: u8 = 0;
        let mut i = 0;
        while i < line.code.len() {
            let c: char = line.code.chars().nth(i).unwrap();
            if i < line.code.len()-1 && c == '&' && line.code.chars().nth(i+1).unwrap() == '[' && in_bracket==false {
                in_bracket = true;
                start_remove_indices.push(i);
                depth = 0;
                i += 1;
            }
            else if c == '[' && in_bracket {
                depth += 1;
            }
            else if c == ']' && in_bracket && depth > 0 {
                depth -= 1;
            }
            else if c == ']' && in_bracket && depth == 0 {
                in_bracket = false;
                end_remove_indices.push(i);
            }
            i += 1;
        }
        // check if lens are valid
        if start_remove_indices.len() != end_remove_indices.len() {
            panic!("Kerneler Error : &[ was never closed\n | {}", line.code);
        }

        // Remove
        for i in (0..start_remove_indices.len()).rev() {
            line.code.replace_range(end_remove_indices[i]..end_remove_indices[i]+1, "");
            line.code.replace_range(start_remove_indices[i]..start_remove_indices[i]+2, "");
        }
    }
}

fn arg_reference(lines: &mut Vec<CodeLine>) {
    for line in lines {
        // Iterate backwards on the string to replace all &smth
        let mut current_word = String::new();
        for i in (0..line.code.len()).rev() {
            let c = line.code.chars().nth(i).unwrap();
            if !(c.is_alphanumeric()||c=='_') {
                if c=='&' {
                    if current_word.is_empty() {
                        panic!("Kerneler Error : Isolated &\n | {}", line.code);
                    }
                    // Replace &current_word with current_word1,current_word2,current_word3
                    line.code.replace_range(i..i+current_word.len()+1, format!("{current_word}$1,{current_word}$2,{current_word}$3").as_str());
                }
                current_word.clear();
            } else {
                current_word.insert(0, c);
            }
        }
    }
}


fn pointers(lines: &mut Vec<CodeLine>) {
    // Replaces [smth1,smth2] to [smth1,smth2,0],[smth1,smth2,1],[smth1,smth2,2]
    for line in lines {
        // Create a stack to push an empty string everytime ] is encountered, and pop the stack when [ is encountered (Pointers can't be in pointers so we can simply replace without caring about the rest of the string getting shifted)
        let mut stack: Vec<String> = Vec::new();
        for i in (0..line.code.len()).rev() {
            let c = line.code.chars().nth(i).unwrap();
            if c == '[' {
                if stack.len() == 0 {
                    panic!("Kerneler Error : Bracket was closed but never opened\n | {}", line.code);
                }
                let string = stack.pop().unwrap();
                // Get the different elements (ignoring comas in brackets)
                let mut things = Vec::new();
                things.push(String::new());
                let mut depth = 0;
                for char in string.chars() {
                    if char == '[' {
                        depth += 1;
                    }
                    else if char == ']' {
                        depth -= 1;
                        if depth < 0 {
                            panic!("Kerneler Error : Bracket was closed but never opened\n | {}", line.code);
                        }
                    }
                    if char == ',' && depth == 0 {
                        things.push(String::new());
                    } else {
                        things.last_mut().unwrap().push(char);
                    }
                }
                if things.len() == 2 {
                    let first = &things[0];
                    let second = &things[1];
                    // Replace
                    line.code.replace_range(i..i+2+string.len(), format!("[{first},{second},0],[{first},{second},1],[{first},{second},2]").as_str());
                }
            }
            if stack.len() > 0 {
                // Add char to every item in the stack
                for string in stack.iter_mut() {
                    string.insert(0, c);
                }
            }
            if c == ']' {
                stack.push(String::new());
            }
        }
    }
}