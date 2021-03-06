pub mod checker;
pub mod parser;

/// A single decoded line in a Make A Lisp reference spec.
#[derive(Clone, Debug)]
pub enum MalTestingLine {
    /// Set all the following lines in the current file as optional or not.
    ///
    /// Optional functionality is not needed for self hosting.
    ToggleOptional(bool),
    /// Declare an exact input and the corresponding exact output after evaluation.
    InputShouldOutput(Vec<String>, String),
    /// Declare an exact input and the corresponding expected error
    /// to occur during evaluation.
    ///
    /// TODO: replace Result<T, String> with more granular error propagation
    /// (eg. failure crate) to better unit test against the spec
    InputShouldThrow(Vec<String>),
    /// (Cosmetic) Declare the current section name for the next lines.
    BeginSection(String),
}
