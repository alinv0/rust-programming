mod demo_box1;
mod demo_box2;
mod demo_box3;
mod demo_rc;
mod demo_unsafe_code;

fn main() {
    demo_box1::do_it();
    demo_box2::do_it();
    demo_box3::do_it();
    demo_rc::do_it();
    demo_unsafe_code::do_it();
}
