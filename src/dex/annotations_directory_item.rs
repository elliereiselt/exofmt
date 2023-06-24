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

use crate::dex::AnnotationItem;
use crate::dex::FieldAnnotation;
use crate::dex::MethodAnnotation;
use crate::dex::ParameterAnnotation;

pub struct AnnotationsDirectoryItem {
    /// Annotations made directly on the class
    pub class_annotations: Vec<AnnotationItem>,
    /// List of associated field annotations
    pub field_annotations: Vec<FieldAnnotation>,
    /// List of associated method annotations
    pub method_annotations: Vec<MethodAnnotation>,
    /// List of associated method parameter annotations
    pub parameter_annotations: Vec<ParameterAnnotation>,
}
