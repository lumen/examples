use liblumen_core::symbols::FunctionSymbol;

use liblumen_alloc::erts::apply::InitializeLumenDispatchTable;

pub fn initialize_dispatch_table(mut additional_function_symbols: Vec<FunctionSymbol>) {
    let mut function_symbols = vec![
        // OTP
        liblumen_otp::erlang::apply_3::function_symbol(),
        // Lumen.Web
        liblumen_web::executor::apply_4::function_symbol(),
        // Crate
        crate::elixir::chain::console_1::function_symbol(),
        crate::elixir::chain::counter_2::function_symbol(),
        crate::elixir::chain::create_processes_2::function_symbol(),
        crate::elixir::chain::dom_1::function_symbol(),
        crate::elixir::chain::none_1::function_symbol(),
        crate::elixir::chain::on_submit_1::function_symbol(),
    ];
    function_symbols.append(&mut additional_function_symbols);

    unsafe {
        InitializeLumenDispatchTable(function_symbols.as_ptr(), function_symbols.len());
    }

    // Don't drop the vec since it needs to be static
    std::mem::forget(function_symbols);
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
