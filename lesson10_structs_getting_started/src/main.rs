mod mytypes;
mod demo_accessing_struct;
mod demo_instances;
mod demo_struct_pass_by_value;
mod demo_struct_pass_by_reference;
mod demo_struct_return_value;
mod demo_struct_return_reference;

fn main() {
    demo_accessing_struct::do_it();
    demo_instances::do_it();
    demo_struct_pass_by_value::do_it();
    demo_struct_pass_by_reference::do_it();
    demo_struct_return_value::do_it();
    demo_struct_return_reference::do_it();
}