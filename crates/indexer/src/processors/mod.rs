// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

pub mod ferum_processor;

use self::{
    ferum_processor::NAME as FERUM_PROCESSOR_NAME
};

pub enum Processor {
    FerumProcessor,
}

impl Processor {
    pub fn from_string(input_str: &String) -> Self {
        match input_str.as_str() {
            FERUM_PROCESSOR_NAME => Self::FerumProcessor,
            _ => panic!("Processor unsupported {}", input_str),
        }
    }
}
