pub trait CamelCase {
    fn camel_case(&self) -> String;
}

impl CamelCase for String {
    fn camel_case(&self) -> String {
        let mut camel = String::new();
        let mut prev_underscore = false;
        for c in self.chars() {
            if c == '_' {
                prev_underscore = true;
            } else if prev_underscore {
                camel.push(c.to_uppercase().next().unwrap());
                prev_underscore = false;
            } else {
                camel.push(c);
            }
        }
        camel
    }
}

impl CamelCase for &str {
    fn camel_case(&self) -> String {
        self.to_string().camel_case()
    }
}
