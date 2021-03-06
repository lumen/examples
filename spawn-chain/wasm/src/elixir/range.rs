use anyhow::*;

use liblumen_alloc::atom;
use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

pub fn new(first: Term, last: Term, process: &Process) -> exception::Result<Term> {
    if first.is_integer() {
        if last.is_integer() {
            Ok(process.map_from_slice(&[
                (atom!("__struct__"), atom!("Elixir.Range")),
                (atom!("first"), first),
                (atom!("last"), last),
            ]))
        } else {
            Err(anyhow!("last ({}) is not an integer", last).into())
        }
    } else {
        if last.is_integer() {
            Err(anyhow!("first ({}) is not an integer", first).into())
        } else {
            Err(anyhow!(
                "neither first ({}) nor last ({}) is an integer",
                first,
                last
            )
            .into())
        }
    }
}
