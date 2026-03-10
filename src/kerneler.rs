
#[derive(Debug)]
struct CodeLine {
    pub code: String,  // The line of code
    pub depth: u8,   // How deep is the line in closures
    pub starts_closure: bool   // Wether this line is a function/procedure signature, if or while
}


pub fn kernel(code: String) -> String {
    let mut lines: Vec<CodeLine> = to_code_lines(code);
    println!("{:?}", lines);


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
                // Remove leading and tailing spaces
                let str = buffer.trim().to_string();
                lines.push(CodeLine { code: str, depth: depth, starts_closure: true}); // Empty lines can be pushed here, this allows the rest of kerneler to know that a new closure was opened, this will be removed on the final parse
                buffer.clear();
                depth += 1;
            } else if char == '}' {
                if depth==0 {
                    panic!("Kerneler Error : Closing a closure that was never opened")
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