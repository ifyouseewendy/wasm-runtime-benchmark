use wasmer_runtime::{
    cache::{Cache, FileSystemCache, WasmHash},
    compile_with, compiler_for_backend, error, imports, Backend, Func, Instance,
};

pub struct Wrapper {
    backend: Backend,
}

#[derive(Debug)]
pub enum AotError {
    Error(wasmer_runtime::error::Error),
    CacheError(wasmer_runtime::error::CacheError),
    CompileError(wasmer_runtime::error::CompileError),
    IOError(std::io::Error),
}
pub type AotResult<T> = std::result::Result<T, AotError>;

impl std::convert::From<std::io::Error> for AotError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
    }
}
impl std::convert::From<wasmer_runtime::error::Error> for AotError {
    fn from(e: wasmer_runtime::error::Error) -> Self {
        Self::Error(e)
    }
}
impl std::convert::From<wasmer_runtime::error::CacheError> for AotError {
    fn from(e: wasmer_runtime::error::CacheError) -> Self {
        Self::CacheError(e)
    }
}
impl std::convert::From<wasmer_runtime::error::CompileError> for AotError {
    fn from(e: wasmer_runtime::error::CompileError) -> Self {
        Self::CompileError(e)
    }
}

impl Wrapper {
    pub fn new(backend: Backend) -> Self {
        Self { backend }
    }

    pub fn jit(&self, wasm_bytes: &[u8], arg: u32) -> error::Result<u32> {
        let compiler = compiler_for_backend(self.backend).unwrap();
        let module = compile_with(wasm_bytes, compiler.as_ref()).unwrap();

        let import_object = imports! {};
        let instance = module.instantiate(&import_object)?;

        let run: Func<u32, u32> = instance.func("ext_run")?;
        let v = run.call(arg)?;
        Ok(v)
    }

    pub fn aot_c(&self, wasm_bytes: &[u8]) -> AotResult<String> {
        let compiler = wasmer_runtime::compiler_for_backend(self.backend).unwrap();
        let module = compile_with(&wasm_bytes, compiler.as_ref())?;

        let mut fs_cache = unsafe { FileSystemCache::new("./tmp/")? };
        let artifact = module.cache()?;
        let key = WasmHash::generate(&artifact.serialize()?);

        fs_cache.store(key, module.clone())?;
        Ok(key.encode())
    }

    pub fn aot_e(&self, key: &str, arg: u32) -> AotResult<u32> {
        let fs_cache = unsafe { FileSystemCache::new("./tmp/")? };
        let module = fs_cache
            .load_with_backend(WasmHash::decode(key).unwrap(), self.backend)
            .unwrap();

        let import_object = imports! {};
        let instance = module.instantiate(&import_object).unwrap();

        let run: Func<u32, u32> = instance.func("ext_run").unwrap();
        let v = run.call(arg).unwrap();
        Ok(v)
    }

    pub fn aot_t(&self, wasm_bytes: &[u8], arg: u32) -> AotResult<u32> {
        let key = self.aot_c(wasm_bytes)?;
        self.aot_e(&key, arg)
    }

    pub fn prepare(&self, wasm_bytes: &[u8]) -> error::Result<Instance> {
        let compiler = compiler_for_backend(self.backend).unwrap();
        let module = compile_with(&wasm_bytes, compiler.as_ref()).unwrap();

        let import_object = imports! {};
        let instance = module.instantiate(&import_object)?;

        Ok(instance)
    }

    pub fn execute(&self, instance: &Instance, arg: u32) -> error::Result<u32> {
        let func: Func<u32, u32> = instance.func("ext_run")?;
        let v = func.call(arg)?;
        Ok(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static WASM: &'static [u8] = include_bytes!("../fibonacci.wasm");

    fn wrapper() -> Wrapper {
        Wrapper::new(Backend::Singlepass)
    }

    #[test]
    fn test_jit() {
        let v = wrapper().jit(&WASM, 5).unwrap();
        assert_eq!(v, 8);
    }
    #[test]
    fn test_aot_t() {
        let v = wrapper().aot_t(&WASM, 5).unwrap();
        assert_eq!(v, 8);
    }
    #[test]
    fn test_call() {
        let wrapper = wrapper();
        let instance = wrapper.prepare(&WASM).unwrap();
        let v = wrapper.execute(&instance, 5).unwrap();
        assert_eq!(v, 8);
    }
}
