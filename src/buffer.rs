use std::ops::RangeBounds;

#[derive(Debug, Clone, Default)]
pub struct Buffer(Vec<String>);

impl Buffer {
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.0.iter()
    }

    pub fn push(&mut self, value: String) {
        self.0.push(value);
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn set_line_content(&mut self, line: usize, content: String) {
        let l = self.0.get_mut(line);
        if let Some(l) = l {
            *l = content;
        }
    }

    pub fn drain<T: RangeBounds<usize>>(&mut self, range: T) {
        self.0.drain(range);
    }
}

impl FromIterator<String> for Buffer {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Buffer(Vec::from_iter(iter))
    }
}
