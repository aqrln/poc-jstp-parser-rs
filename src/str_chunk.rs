#[derive(Debug, Copy, Clone)]
pub enum StrChunk<'a> {
    Slice(&'a str),
    Char(char),
}

impl<'a> StrChunk<'a> {
    pub fn len(&self) -> usize {
        match *self {
            StrChunk::Slice(s) => s.len(),
            StrChunk::Char(c) => c.len_utf8(),
        }
    }
}
