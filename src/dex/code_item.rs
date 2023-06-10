use crate::dex::DebugInfoItem;
use crate::dex::EncodedCatchHandler;
use crate::dex::TryItem;

pub struct CodeItem {
    /// the number of registers used by this code
    pub registers_size: u16,
    /// Line numbers and local variable info for the code item
    pub debug_info: Option<DebugInfoItem>,
    /// Actual array of bytecode. The format of code in an insns array is specified by [Dalvik bytecode](https://source.android.com/docs/core/runtime/dalvik-bytecode)
    pub instructions: Vec<u16>,
    /// Array indicating where in the code exceptions are caught and how to handle them.
    /// Elements of the array must be non-overlapping in range and in order from low to high address
    pub tries: Vec<TryItem>,
    /// List of lists of catch types and associated handler addresses
    pub handlers: Vec<EncodedCatchHandler>,
}
