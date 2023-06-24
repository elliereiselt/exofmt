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
    /// Vector of strings which are not present in the corresponding Dex file
    /// These are referring to IDs starting with `string_ids.len()` of the Dex file
    pub strings: Vec<CString>,
    /// Set of class pairs recording the assignability test from one type to the other
    pub assignable_types: Vec<TypeAssignability>,
    /// Set of class pairs recording the assignability test from one type to the other
    pub unassignable_types: Vec<TypeAssignability>,
    /// Set of class resolutions
    pub classes: Vec<ClassResolution>,
    /// Set of field resolutions
    pub fields: Vec<FieldResolution>,
    /// Set of method resoliutions
    pub methods: Vec<MethodResolution>,
    /// Bit vector indexed by class def indices indicating whether the corresponding
    /// class was successfully verified
    pub verified_classes: Vec<bool>,
    /// Bit vector indexed by class def indices indicating whether the corresponding
    /// class resolved into a different class with the same descriptor (was eclipsed).
    /// The other class might have been both external (not covered by these VerifierDeps)
    /// and internal (same VerifierDeps, different DexFileDeps).
    pub redefined_classes: Vec<bool>,
}

pub struct TypeAssignability {
    pub destination_index: u32,
    pub source_index: u32,
}

pub struct ClassResolution {
    pub type_index: u16,
    pub access_flags: u16,
}

pub struct FieldResolution {
    pub field_index: u32,
    pub access_flags: u16,
    pub declaring_class_index: u32,
}

pub struct MethodResolution {
    pub method_index: u32,
    pub access_flags: u16,
    pub declaring_class_index: u32,
}
