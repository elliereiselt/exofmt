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

// All of this is ugly and brittle so just be careful.
// They break easily but make life easier so :shrug:
macro_rules! stringable_consts_block {
    (
        const stringable: $stringable_const_type:ty {
            $(
                $(#[$stringable_meta:meta $($stringable_args:tt)*])*
                $stringable_vis:vis $stringable_name:ident = $stringable_value:expr;
            )*
        }

        const ignore: $ignore_const_type:ty {
            $(
                $(#[$ignore_meta:meta $($ignore_args:tt)*])*
                $ignore_vis:vis $ignore_name:ident = $ignore_value:expr;
            )*
        }

        $(#[$fn_meta:meta $($fn_meta_args:tt)*])*
        $fn_vis:vis fn $fn_name:ident(value: $func_type:ty) -> &'static str {
            match value {
                $unknown:ident => $result_value:expr $(,)?
            }
        }
    ) => {
        $(
            $(#[$stringable_meta $($stringable_args)*])*
            $stringable_vis const $stringable_name : $stringable_const_type = $stringable_value;
        )*

        $(
            $(#[$ignore_meta $($ignore_args)*])*
            $ignore_vis const $ignore_name : $ignore_const_type = $ignore_value;
        )*

        $(#[$fn_meta $($fn_meta_args)*])*
        $fn_vis fn $fn_name(value: $func_type) -> &'static str {
            match value {
                $(
                    $stringable_name => std::stringify!($stringable_name),
                )*
                $unknown => $result_value,
            }
        }
    };
}

macro_rules! option_stringable_consts_block {
    (
        const stringable: $stringable_const_type:ty {
            $(
                $(#[$stringable_meta:meta $($stringable_args:tt)*])*
                $stringable_vis:vis $stringable_name:ident = $stringable_value:expr;
            )*
        }

        const ignore: $ignore_const_type:ty {
            $(
                $(#[$ignore_meta:meta $($ignore_args:tt)*])*
                $ignore_vis:vis $ignore_name:ident = $ignore_value:expr;
            )*
        }

        $(#[$fn_meta:meta $($fn_meta_args:tt)*])*
        $fn_vis:vis fn $fn_name:ident(value: $func_type:ty) -> Option<&'static str>;
    ) => {
        $(
            $(#[$stringable_meta $($stringable_args)*])*
            $stringable_vis const $stringable_name : $stringable_const_type = $stringable_value;
        )*

        $(
            $(#[$ignore_meta $($ignore_args)*])*
            $ignore_vis const $ignore_name : $ignore_const_type = $ignore_value;
        )*

        $(#[$fn_meta $($fn_meta_args)*])*
        $fn_vis fn $fn_name(value: $func_type) -> Option<&'static str> {
            match value {
                $(
                    $stringable_name => Some(std::stringify!($stringable_name)),
                )*
                _ => None,
            }
        }
    };
}

pub(crate) use option_stringable_consts_block;
pub(crate) use stringable_consts_block;
