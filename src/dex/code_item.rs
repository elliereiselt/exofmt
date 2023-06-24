/*
 * Copyright 2023 Ellie Reiselt
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::dex::DebugInfoItem;
use crate::dex::EncodedCatchHandler;
use crate::dex::TryItem;

pub struct CodeItem {
    /// The number of registers used by this code
    pub registers_size: u16,
    /// The number of words of incoming arguments to the method that this code is for
    pub ins_size: u16,
    /// The number of words of outgoing argument space required by this code for method invocation
    pub outs_size: u16,
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
