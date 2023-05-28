#[derive(PartialEq)]
pub enum LineEditorState {
    NewWord,
    Comment,
    Annotation,
    StringLiteral,
    EscapedChar,
    ObjectReading(String), // object name
}