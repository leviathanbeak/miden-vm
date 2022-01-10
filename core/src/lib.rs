pub mod hasher;
pub mod program;
pub mod utils;
pub use math::{fields::f64::BaseElement, FieldElement, StarkField};

mod operations;
pub use operations::Operation;

mod inputs;
pub use inputs::{AdviceSet, ProgramInputs};

pub mod errors;

// TYPE ALIASES
// ================================================================================================

pub type Word = [BaseElement; 4];

// CONSTANTS
// ================================================================================================

/// Number of stack registers which can be accesses by the VM directly.
pub const STACK_TOP_SIZE: usize = 16;
