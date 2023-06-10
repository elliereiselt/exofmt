use crate::dex::uleb128p1::uleb128p1;
use crate::stringable_consts_blocks::stringable_consts_block;

pub struct DebugInfoItem {
    /// Ihe initial value for the state machine's line register. Does not represent an actual positions entry
    pub line_start: u32,
    /// String index of the method parameter name. An encoded value of NO_INDEX indicates that no name is
    /// available for the associated parameter. The type descriptor and signature are implied from the method descriptor and signature
    pub parameters: Vec<uleb128p1>,
    /// Actual bytecode for the state machine
    pub bytecode: Vec<u8>,
}

stringable_consts_block! {
    const stringable: u8 {
        /// Terminates a debug info sequence for a `code_item`
        pub DBG_END_SEQUENCE = 0x00;
        /// Advances the address register without emitting a positions entry
        ///
        /// Format: `uleb128 addr_diff`
        /// Arguments:
        ///     - `addr_diff`: amount to add to address register
        pub DBG_ADVANCE_PC = 0x01;
        /// Advances the line register without emitting a positions entry
        ///
        /// Format: `sleb128 line_diff`
        /// Arguments:
        ///     - `line_diff`: amount to change line register by
        pub DBG_ADVANCE_LINE = 0x02;
        /// Introduces a local variable at the current address.
        /// Either `name_idx` or `type_idx` may be `NO_INDEX` to indicate that that value is unknown
        ///
        /// Format:
        ///     ```
        ///     uleb128 register_num
        ///     uleb128p1 name_idx
        ///     uleb128p1 type_idx
        ///     ```
        /// Arguments:
        ///     - `register_num`: register that will contain local
        ///     - `name_idx`: string index of the name
        ///     - `type_idx`: type index of the type
        pub DBG_START_LOCAL = 0x03;
        /// Introduces a local with a type signature at the current address.
        /// Any of `name_idx`, `type_idx`, or `sig_idx` may be `NO_INDEX` to indicate that that value is unknown.
        /// (If `sig_idx` is `-1`, though, the same data could be represented more efficiently using the opcode `DBG_START_LOCAL`)
        ///
        /// Note: See discussion under [dalvik.annotation.Signature](https://source.android.com/docs/core/runtime/dex-format#dalvik-signature)
        ///       for caveats about handling signatures
        ///
        /// Format:
        ///     ```
        ///     uleb128 register_num
        ///     uleb128p1 name_idx
        ///     uleb128p1 type_idx
        ///     uleb128p1 sig_idx
        ///     ```
        /// Arguments:
        ///     - `register_num`: register that will contain local
        ///     - `name_idx`: string index of the name
        ///     - `type_idx`: type index of the type
        ///     - `sig_idx`: string index of the type signature
        pub DBG_START_LOCAL_EXTENDED = 0x04;
        /// Marks a currently-live local variable as out of scope at the current address
        ///
        /// Format: `uleb128 register_num`
        /// Arguments:
        ///     - `register_num`: register that contained local
        pub DBG_END_LOCAL = 0x05;
        /// Re-introduces a local variable at the current address.
        /// The name and type are the same as the last local that was live in the specified register
        ///
        /// Format: `uleb128 register_num`
        /// Arguments:
        ///     - `register_num`: register to restart
        pub DBG_RESTART_LOCAL = 0x06;
        /// Sets the `prologue_end` state machine register, indicating that the next position entry that
        /// is added should be considered the end of a method prologue (an appropriate place for a method breakpoint).
        /// The `prologue_end` register is cleared by any special (>= `0x0a`) opcode
        pub DBG_SET_PROLOGUE_END = 0x07;
        /// Sets the `epilogue_begin` state machine register, indicating that the next position entry that
        /// is added should be considered the beginning of a method epilogue (an appropriate place to suspend execution before method exit).
        /// The `epilogue_begin` register is cleared by any special (>= `0x0a`) opcode
        pub DBG_SET_EPILOGUE_BEGIN = 0x08;
        /// Indicates that all subsequent line number entries make reference to this source file name, instead of the default name specified in `code_item`
        pub DBG_SET_FILE = 0x09;
    }

    const ignore: u8 {
        /// Start of special opcodes. Special opcodes move both the `line` and `address` registers by a small amount and then emit a new position table entry
        ///
        /// See: https://source.android.com/docs/core/runtime/dex-format#opcodes
        pub DBG_FIRST_SPECIAL = 0x0a;
        /// End of special opcodes. Special opcodes move both the `line` and `address` registers by a small amount and then emit a new position table entry
        ///
        /// See: https://source.android.com/docs/core/runtime/dex-format#opcodes
        pub DBG_LAST_SPECIAL = 0xff;
    }

    pub fn dbg_to_str(value: u8) -> &'static str {
        match value {
            unknown => {
                if unknown >= DBG_FIRST_SPECIAL && unknown <= DBG_LAST_SPECIAL {
                    "DBG_SPECIAL"
                } else {
                    "DBG_UNKNOWN"
                }
            }
        }
    }
}
