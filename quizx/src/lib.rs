// QuiZX - Rust library for quantum circuit rewriting and optimisation
//         using the ZX-calculus
// Copyright (C) 2021 - Aleks Kissinger
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod annealer;
pub mod basic_rules;
pub mod circuit;
pub mod decompose;
pub mod extract;
pub mod gate;
pub mod generate;
pub mod graph;
pub mod hash_graph;
pub mod linalg;
pub mod optimize_circuit;
pub mod random_graph;
pub mod scalar;
pub mod simplify;
pub mod tensor;
pub mod vec_graph;

pub mod prelude {
    pub use crate::{
        annealer::*, circuit::*, extract::*, graph::*, hash_graph::*, scalar::*, simplify::*,
    };
}
