//! ```elixir
//! pushed from environment: (output)
//! pushed from arguments: (element, send_to)
//! pushed to stack: (output, element, send_to)
//! returned from call: N/A
//! full stack: (output, element, send_to)
//! fn (_, send_to) ->
//!   spawn(Chain, :counter, [send_to, output])
//! end
//! ```

use std::convert::TryInto;

use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

use liblumen_otp::erlang;

pub fn closure(process: &Process, output: Term) -> Term {
    process.anonymous_closure_with_env_from_slice(
        super::module(),
        0,
        Default::default(),
        Default::default(),
        2,
        CLOSURE_NATIVE,
        process.pid().into(),
        &[output],
    )
}

// Private

#[native_implemented::function(Elixir.Chain:create_processes_reducer/3)]
fn result(
    process: &Process,
    closure: Term,
    element: Term,
    send_to: Term,
) -> exception::Result<Term> {
    let closure_boxed_closure: Boxed<Closure> = closure.try_into().unwrap_or_else(|_| {
        panic!(
            "First argument ({}) passed to closure with environment should be closure",
            closure
        )
    });
    let env_slice = closure_boxed_closure.env_slice();
    assert_eq!(env_slice.len(), 1);
    let output = env_slice[0];

    // from environment
    assert!(
        output.is_boxed_function(),
        "Output ({:?}) is not a function",
        output
    );

    // from arguments
    assert!(
        element.is_integer(),
        "element ({}) is not an integer",
        element
    );
    assert!(send_to.is_pid());

    let module = Atom::str_to_term("Elixir.Chain");
    let function = Atom::str_to_term("counter");
    let arguments = process.list_from_slice(&[send_to, output]);

    process.queue_frame_with_arguments(
        erlang::spawn_3::frame().with_arguments(false, &[module, function, arguments]),
    );

    Ok(Term::NONE)
}
