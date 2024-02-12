pub trait SnakeCase {
    fn snake_case(&self) -> String;
}

impl SnakeCase for String {
    fn snake_case(&self) -> String {
        let mut snake = String::new();
        let mut prev_upper = false;
        for c in self.chars() {
            if c.is_uppercase() {
                if prev_upper {
                    snake.push(c.to_lowercase().next().unwrap());
                } else {
                    snake.push('_');
                    snake.push(c.to_lowercase().next().unwrap());
                }
                prev_upper = true;
            } else {
                snake.push(c);
                prev_upper = false;
            }
        }
        snake
    }
}

impl SnakeCase for &str {
    fn snake_case(&self) -> String {
        self.to_string().snake_case()
    }
}
