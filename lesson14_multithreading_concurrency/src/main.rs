mod demo_spawning_threads;
mod demo_join_thread_single;
mod demo_join_thread_multiple;
mod demo_capturing_state_implicit_move;
mod demo_capturing_state_explicit_move;
mod demo_channel_single_message;
mod demo_channel_multiple_messages;

fn main() {
    // demo_spawning_threads::do_it();
    // demo_join_thread_single::do_it();
    // demo_join_thread_multiple::do_it();
    // demo_capturing_state_implicit_move::do_it();
    // demo_capturing_state_explicit_move::do_it();
    // demo_channel_single_message::do_it();
    demo_channel_multiple_messages::do_it();
}
