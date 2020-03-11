use lucet_runtime::{DlModule, Limits, MmapRegion, Region};
use lucet_wasi::WasiCtxBuilder;

pub fn run(file: &str, arg: u32) -> u32 {
    lucet_runtime::lucet_internal_ensure_linked();
    // ensure the WASI symbols are exported from the final executable
    lucet_wasi::export_wasi_funcs();
    // load the compiled Lucet module
    let dl_module = DlModule::load(file).unwrap();

    // create a new memory region with default limits on heap and stack size
    let region = MmapRegion::create(
        1,
        &Limits {
            heap_memory_size: 8 * 1024 * 1024,
            stack_size: 128 * 1024,
            ..Limits::default()
        },
    )
    .unwrap();

    // instantiate the module in the memory region
    let mut instance = region.new_instance(dl_module).unwrap();

    // prepare the WASI context, inheriting stdio handles from the host executable
    // let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build().unwrap();
    // instance.insert_embed_ctx(wasi_ctx);

    // run the WASI main function
    instance
        .run("ext_run", &[arg.into()])
        .unwrap()
        .returned()
        .unwrap()
        .as_u32()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let v = run("fibonacci.so", 20);
        println!("Result is {:?}", v);
    }
}
