use wasmer_runtime::cache::{Cache, FileSystemCache, WasmHash};
use wasmer_runtime::{compile_with, error, imports, instantiate, Func, Module};

// Singlepass
pub fn jit_with_singlepass(file: &str, arg: u32) -> error::Result<u32> {
    jit(wasmer_runtime::Backend::Singlepass, file, arg)
}
pub fn store_with_singlepass(file: &str) -> Result<String, error::CacheError> {
    store_module(wasmer_runtime::Backend::Singlepass, file)
}
pub fn aot_with_singlepass(key: &str, arg: u32) -> error::Result<u32> {
    aot(wasmer_runtime::Backend::Singlepass, key, arg)
}

// Cranelift
pub fn jit_with_cranelift(file: &str, arg: u32) -> error::Result<u32> {
    jit(wasmer_runtime::Backend::Cranelift, file, arg)
}
pub fn store_with_cranelift(file: &str) -> Result<String, error::CacheError> {
    store_module(wasmer_runtime::Backend::Cranelift, file)
}
pub fn aot_with_cranelift(key: &str, arg: u32) -> error::Result<u32> {
    aot(wasmer_runtime::Backend::Cranelift, key, arg)
}

// LLVM
pub fn jit_with_llvm(file: &str, arg: u32) -> error::Result<u32> {
    jit(wasmer_runtime::Backend::LLVM, file, arg)
}
pub fn store_with_llvm(file: &str) -> Result<String, error::CacheError> {
    store_module(wasmer_runtime::Backend::LLVM, file)
}
pub fn aot_with_llvm(key: &str, arg: u32) -> error::Result<u32> {
    aot(wasmer_runtime::Backend::LLVM, key, arg)
}

pub fn jit(backend: wasmer_runtime::Backend, file: &str, arg: u32) -> error::Result<u32> {
    let key = store_module(backend, file).unwrap();
    // println!("key is {:?}", &key);

    let module = load_module(backend, &key).unwrap();

    let import_object = imports! {};
    let instance = module.instantiate(&import_object)?;
    let run: Func<u32, u32> = instance.func("ext_run")?;
    let v = run.call(arg)?;
    Ok(v)
}
pub fn aot(backend: wasmer_runtime::Backend, key: &str, arg: u32) -> error::Result<u32> {
    let module = load_module(backend, key).unwrap();

    let import_object = imports! {};
    let instance = module.instantiate(&import_object)?;
    let run: Func<u32, u32> = instance.func("ext_run")?;
    let v = run.call(arg)?;
    Ok(v)
}

pub fn store_module(
    backend: wasmer_runtime::Backend,
    file: &str,
) -> Result<String, error::CacheError> {
    // let wasm_bytes = include_bytes!(file.to_own());

    let wasm_bytes = std::fs::read(std::path::Path::new(file)).unwrap();

    let compiler = wasmer_runtime::compiler_for_backend(backend).unwrap();
    let module = compile_with(&wasm_bytes, compiler.as_ref()).unwrap();

    // Create a new file system cache.
    // This is unsafe because we can't ensure that the artifact wasn't
    // corrupted or tampered with.
    let mut fs_cache = unsafe { FileSystemCache::new("./tmp/")? };

    let artifact = module.cache().unwrap();
    let key = WasmHash::generate(&artifact.serialize().unwrap());
    // Store a module into the cache given a key
    fs_cache.store(key, module.clone())?;
    Ok(key.encode())
}

fn load_module(backend: wasmer_runtime::Backend, key: &str) -> Result<Module, error::CacheError> {
    let fs_cache = unsafe { FileSystemCache::new("./tmp/")? };
    fs_cache.load_with_backend(WasmHash::decode(key).unwrap(), backend)
}

// jit_with_singlepass measures the whole process of compilation and execution
// aot_with_singlepass measures the execution of pre-compiled machine code

// pub fn with_cranelift(num: u32) -> error::Result<()> {
//     let wasm_bytes = include_bytes!("../fibonacci.wasm");
//     let module = compile_with(wasm_bytes, &CraneliftCompiler::new()).unwrap();
//     let import_object = imports! {};
//     let instance = module.instantiate(&import_object)?;
//     let run: Func<u32, u32> = instance.func("ext_run")?;
//     let _ = run.call(num)?;
//     Ok(())
// }
//
// pub fn with_singlepass(num: u32) -> error::Result<()> {
//     let wasm_bytes = include_bytes!("../fibonacci.wasm");
//     let module = compile_with(wasm_bytes, &SinglePassCompiler::new()).unwrap();
//     let import_object = imports! {};
//     let instance = module.instantiate(&import_object)?;
//     let run: Func<u32, u32> = instance.func("ext_run")?;
//     let _ = run.call(num)?;
//     Ok(())
// }

// rename to jit_with_llvm
// pub fn with_llvm(num: u32) -> error::Result<()> {
//     let wasm_bytes = include_bytes!("../fibonacci.wasm");
//     let module = compile(wasm_bytes).unwrap();
//
//     let import_object = imports! {};
//     let instance = module.instantiate(&import_object)?;
//     let run: Func<u32, u32> = instance.func("ext_run")?;
//     let _ = run.call(num)?;
//     Ok(())
// }
//
// pub fn aot_with_llvm(num: u32, key: &str) -> error::Result<u32> {
//     let module = load_module(key).unwrap();
//
//     let import_object = imports! {};
//     let instance = module.instantiate(&import_object)?;
//     let run: Func<u32, u32> = instance.func("ext_run")?;
//     let v = run.call(num)?;
//     Ok(v)
// }
//
// pub fn store_module() -> Result<String, error::CacheError> {
//     let wasm_bytes = include_bytes!("../fibonacci.wasm");
//     let module = compile(wasm_bytes).unwrap();
//
//     // Create a new file system cache.
//     // This is unsafe because we can't ensure that the artifact wasn't
//     // corrupted or tampered with.
//     let mut fs_cache = unsafe { FileSystemCache::new("./tmp/")? };
//     // Compute a key for a given WebAssembly binary
//
//     let artifact = module.cache().unwrap();
//     let key = WasmHash::generate(&artifact.serialize().unwrap());
//     // Store a module into the cache given a key
//     fs_cache.store(key, module.clone())?;
//     Ok(key.encode())
// }
//
// fn load_module(key: &str) -> Result<Module, error::CacheError> {
//     let fs_cache = unsafe { FileSystemCache::new("./tmp/")? };
//     fs_cache.load(WasmHash::decode(key).unwrap())
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jit_with_singlepass() {
        let v = jit_with_singlepass("./fibonacci.wasm", 10).unwrap();
        println!("Result is {}", v);
    }
    #[test]
    fn test_aot_with_singlepass() {
        let key = store_with_singlepass("./fibonacci.wasm").unwrap();
        let v = aot_with_singlepass(&key, 10).unwrap();
        println!("Result is {}", v);
    }

    #[test]
    fn test_jit_with_cranelift() {
        let v = jit_with_cranelift("./fibonacci.wasm", 10).unwrap();
        println!("Result is {}", v);
    }
    #[test]
    fn test_aot_with_cranelift() {
        let key = store_with_cranelift("./fibonacci.wasm").unwrap();
        let v = aot_with_cranelift(&key, 10).unwrap();
        println!("Result is {}", v);
    }

    #[test]
    fn test_jit_with_llvm() {
        let v = jit_with_llvm("./fibonacci.wasm", 10).unwrap();
        println!("Result is {}", v);
    }
    #[test]
    fn test_aot_with_llvm() {
        let key = store_with_llvm("./fibonacci.wasm").unwrap();
        let v = aot_with_llvm(&key, 10).unwrap();
        println!("Result is {}", v);
    }
}
