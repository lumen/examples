//! ```elixir
//! defp console_output(text) do
//!   IO.puts("#{self()} #{text}")
//! end
//! ```

mod label_1;

use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

use liblumen_otp::erlang;

pub fn closure(process: &Process) -> Term {
    process.export_closure(function(), super::module(), ARITY, CLOSURE_NATIVE)
}

#[native_implemented::function(Elixir.Chain:console_output/1)]
fn result(process: &Process, text: Term) -> Term {
    process.queue_frame_with_arguments(erlang::self_0::frame().with_arguments(false, &[]));
    process.queue_frame_with_arguments(label_1::frame().with_arguments(true, &[text]));

    Term::NONE
}
