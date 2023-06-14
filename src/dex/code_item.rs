// exofmt - binary format parser for ELF, Dex, and more.
// Copyright (C) 2023  Ellie Reiselt
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
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
