use lucet_runtime::{DlModule, InstanceHandle, Limits, MmapRegion, Region};
use lucetc::{Lucetc, LucetcOpts};
use multibase::{encode, Base};

// What if we don't save it to file, but in a memory array?
// aka, what's the overhead for using file IO
// aka, what's the performance for simulated JIT
pub fn aot_c(wasm_bytes: &[u8]) -> String {
    let moduleid = encode(Base::Base58Btc, &wasm_bytes[0..30]);

    let path = format!("./tmp/lucet/{}", moduleid);
    let output_path = std::path::Path::new(&path);
    let compiler = Lucetc::try_from_bytes(wasm_bytes)
        .unwrap()
        .with_opt_level(lucetc::OptLevel::Speed);
    compiler.shared_object_file(&output_path).unwrap();

    moduleid
}

pub fn aot_e(moduleid: &str, arg: u32) -> u32 {
    lucet_runtime::lucet_internal_ensure_linked();
    let dl_module = DlModule::load(format!("./tmp/lucet/{}", moduleid)).unwrap();

    let region = MmapRegion::create(
        1,
        &Limits {
            heap_memory_size: 8 * 1024 * 1024,
            stack_size: 128 * 1024,
            ..Limits::default()
        },
    )
    .unwrap();

    let mut instance = region.new_instance(dl_module).unwrap();

    instance
        .run("ext_run", &[arg.into()])
        .unwrap()
        .returned()
        .unwrap()
        .as_u32()
}

pub fn aot_t(wasm_bytes: &[u8], arg: u32) -> u32 {
    let moduleid = aot_c(wasm_bytes);
    aot_e(&moduleid, arg)
}

pub fn compile(wasm_bytes: &[u8]) -> String {
    let moduleid = encode(Base::Base58Btc, &wasm_bytes[0..30]);

    let path = format!("./tmp/lucet/{}", moduleid);
    let output_path = std::path::Path::new(&path);
    let compiler = Lucetc::try_from_bytes(wasm_bytes)
        .unwrap()
        .with_opt_level(lucetc::OptLevel::Speed);
    compiler.shared_object_file(&output_path).unwrap();
    moduleid
}

pub fn instantiate(moduleid: &str) -> InstanceHandle {
    // I wonder how much overhead it is to write and read through file.
    // How about changing it to memory
    lucet_runtime::lucet_internal_ensure_linked();
    let dl_module = DlModule::load(format!("./tmp/lucet/{}", moduleid)).unwrap();

    let region = MmapRegion::create(
        1,
        &Limits {
            heap_memory_size: 8 * 1024 * 1024,
            stack_size: 128 * 1024,
            ..Limits::default()
        },
    )
    .unwrap();

    region.new_instance(dl_module).unwrap()
}

pub fn prepare(wasm_bytes: &[u8]) -> InstanceHandle {
    let moduleid = compile(wasm_bytes);
    instantiate(&moduleid)
}

pub fn call(instance: &mut InstanceHandle, arg: u32) -> u32 {
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

    static WASM: &'static [u8] = include_bytes!("../wasm-sample/fibonacci.wasm");

    #[test]
    fn test_aot_c() {
        let moduleid = aot_c(&WASM);
        println!("moduleid is {:?}", moduleid);
    }

    #[test]
    fn test_aot_t() {
        assert_eq!(aot_t(&WASM, 10), 89);
    }

    #[test]
    fn test_call() {
        let mut instance = prepare(&WASM);
        assert_eq!(call(&mut instance, 10), 89);
    }
}
