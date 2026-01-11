#[derive(Debug, Clone, Default)]
pub struct Buffer(Vec<String>);

impl Buffer {
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.0.iter()
    }

    pub fn push(&mut self, value: String) {
        self.0.push(value);
    }
}
