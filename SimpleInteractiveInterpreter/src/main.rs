struct Interpreter {
    num: f32,
    vars: Vec<(String, f32)>
}

impl Interpreter {
    fn new() -> Interpreter {
        Interpreter {
            num: 0.0,
            vars: Vec::new()
        }
    }

    fn input(&mut self, input: &str) -> Result<Option<f32>, String> {
        for i in input
    }
}
