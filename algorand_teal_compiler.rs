//! Algorand TEAL 合约编译器
pub struct TEALCompiler {
    code: Vec<String>,
}

impl TEALCompiler {
    pub fn new() -> Self {
        Self { code: Vec::new() }
    }

    pub fn add_op(&mut self, op: &str) {
        self.code.push(op.into());
    }

    pub fn compile(&self) -> String {
        self.code.join("\n")
    }
}

fn main() {
    let mut compiler = TEALCompiler::new();
    compiler.add_op("int 0");
    println!("Compiled: {}", compiler.compile());
}
