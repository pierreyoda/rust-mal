pub mod checker;
pub mod parser;

#[derive(Clone, Debug)]
pub enum MalTestingLine {
    /// Set all the following lines in the current file as optional or not.
    ///
    /// Optional functionality is not needed for self hosting.
    ToggleOptional(bool),
    /// Declare an exact input and the corresponding exact output after evaluation.
    InputShouldOutput(Vec<String>, String),
    /// (Cosmetic) Declare the current section name for the next lines.
    BeginSection(String),
}
