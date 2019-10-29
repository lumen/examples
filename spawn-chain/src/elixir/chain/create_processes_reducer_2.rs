use std::sync::Arc;

use liblumen_alloc::erts::exception::Alloc;
use liblumen_alloc::erts::process::code::stack::frame::Placement;
use liblumen_alloc::erts::process::{code, Process};
use liblumen_alloc::erts::term::prelude::*;
use liblumen_alloc::erts::ModuleFunctionArity;

use lumen_runtime::otp::erlang;

pub fn closure(process: &Process, output: Term) -> std::result::Result<Term, Alloc> {
    process.closure_with_env_from_slice(
        module_function_arity(),
        code,
        process.pid_term(),
        &[output],
    )
}

// Private

/// ```elixir
/// pushed from environment: (output)
/// pushed from arguments: (element, send_to)
/// pushed to stack: (output, element, send_to)
/// returned from call: N/A
/// full stack: (output, element, send_to)
/// fn (_, send_to) ->
///   spawn(Chain, :counter, [send_to, output])
/// end
/// ```
fn code(arc_process: &Arc<Process>) -> code::Result {
    arc_process.reduce();

    // from environment
    let output = arc_process.stack_pop().unwrap();
    assert!(output.is_function());
    // from arguments
    let element = arc_process.stack_pop().unwrap();
    assert!(element.is_integer());
    let send_to = arc_process.stack_pop().unwrap();
    assert!(send_to.is_pid());

    // In `lumen` compiled code the compile would optimize this to a direct call of
    // `Scheduler::spawn(arc_process, module, function, arguments, counter_0_code)`, but we want
    // to demonstrate the the `lumen_runtime::code::set_apply_fn` system works here.

    let module = Atom::str_to_term("Elixir.Chain");
    let function = Atom::str_to_term("counter");
    let arguments = arc_process.list_from_slice(&[send_to, output])?;
    erlang::spawn_3::place_frame_with_arguments(
        arc_process,
        Placement::Replace,
        module,
        function,
        arguments,
    )?;

    Process::call_code(arc_process)
}

fn function() -> Atom {
    Atom::try_from_str("create_processes_reducer").unwrap()
}

fn module_function_arity() -> Arc<ModuleFunctionArity> {
    Arc::new(ModuleFunctionArity {
        module: super::module(),
        function: function(),
        arity: 2,
    })
}
