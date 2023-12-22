mod demo_locals;
mod demo_static_locals;
mod demo_static_global;
mod demo_static_mutable;
mod demo_string_handling;
mod demo_copying_and_moving;
mod demo_cloning;

use demo_static_global::GLOBAL_MESSAGE;

fn main() {
    demo_locals::do_it();
    demo_static_locals::do_it();
    demo_static_global::do_it();
    demo_static_mutable::do_it();
    demo_string_handling::do_it();
    demo_copying_and_moving::do_it();
    demo_cloning::do_it();

    println!("\nGLOBAL_MESSAGE: {}", GLOBAL_MESSAGE)
}
