//! # GPT
//!
//! GUID Partition Table parsing.

pub mod entry;
pub mod header;
pub mod parser;
pub mod validation;

pub use entry::{GptEntry, is_unused, parse_gpt_entry};
pub use header::{GPT_SIGNATURE, GptHeader, parse_gpt_header};
pub use parser::{GptIter, gpt_iter, non_empty_entries};
pub use validation::{entry_array_size, validate_header_lbas};
