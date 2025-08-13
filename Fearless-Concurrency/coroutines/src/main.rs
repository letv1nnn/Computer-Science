#![feature(coroutines)]
#![feature(coroutine_trait)]
#![allow(unused)]

use crate::{coroutine_basic_example::run_coroutine_example, coroutines_stacking::coroutine_stack_example, simple_generator::simple_generator_example};

mod coroutine_basic_example;
mod simple_generator;
mod coroutines_stacking;

fn main() {
    // basic example, comparison of default writing into the file and leveraging coroutines
    // run_coroutine_example();

    // implementing a simple generator
    // simple_generator_example();

    // stacking out coroutines
    // coroutine_stack_example();
}