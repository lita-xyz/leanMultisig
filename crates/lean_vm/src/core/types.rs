use p3_koala_bear::{KoalaBear, QuinticExtensionFieldKB};
use std::cmp::Ordering;
use derive_more::Display;

/// Base field type for VM operations
pub type F = KoalaBear;

/// Extension field type for VM operations
pub type EF = QuinticExtensionFieldKB;

/// Line number in source code for debugging
pub type SourceLineNumber = usize;

/// Bytecode address (i.e., a value of the program counter)
pub type CodeAddress = usize;

/// Memory address
pub type MemoryAddress = usize;

/// Source code function name
pub type FunctionName = String;

/// Unique identifier for a file in a compilation
pub type FileId = usize;

/// Location in source code
#[derive(Display, Hash, PartialEq, Eq, Ord, Debug, Clone, Copy)]
#[display("{}:{}", file_id, line_number)]
pub struct SourceLocation {
    pub file_id: FileId,
    pub line_number: SourceLineNumber,
}

impl PartialOrd for SourceLocation {
    fn partial_cmp(&self, other: &SourceLocation) -> Option<Ordering> {
        match self.file_id.cmp(&other.file_id) {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Equal =>
                Some(self.line_number.cmp(&other.line_number))
        }
    }
}
