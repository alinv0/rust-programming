mod demo_simple_impl;
mod demo_mutable_impl;
mod mytypes;
mod demo_modular_code;
mod demo_associated_functions;
mod demo_associated_data;

fn main() {
    demo_simple_impl::do_it();
    demo_mutable_impl::do_it();
    demo_associated_functions::do_it();
    demo_associated_data::do_it();
}
