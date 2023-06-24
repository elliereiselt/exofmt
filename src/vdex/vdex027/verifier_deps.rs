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

use std::ffi::CString;

pub struct DexFileDeps {
    /// Vector of strings which are not present in the corresponding DEX file.
    /// These are referred to with ids starting with `NumStringIds()` of that DexFile.
    pub strings: Vec<CString>,
    /// Vector that contains for each class def defined in a dex file, a set of class pairs recording
    /// the outcome of assignability test from one of the two types to the other.
    pub assignable_types: Vec<Vec<TypeAssignability>>,
    /// Bit vector indexed by class def indices indicating whether the corresponding
    /// class was successfully verified.
    pub verified_classes: Vec<bool>,
}

pub struct TypeAssignability {
    pub destination_index: u32,
    pub source_index: u32,
}
