use p3_koala_bear::{KoalaBear, QuinticExtensionFieldKB};

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
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct SourceLocation {
    pub file_id: FileId,
    pub line_number: SourceLineNumber,
}
