use std::path::Path;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::{InitializationConfig, Target};

pub struct LLVMCreator {
    pub context: Context,
    pub builder: Builder,
    pub module: Module,
}

impl LLVMCreator {
    pub fn new(module_name: &str) -> LLVMCreator {
        Target::initialize_native(&InitializationConfig::default()).unwrap();

        let context = Context::create();
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        LLVMCreator {
            builder: builder,
            module: module,
            context: context,
        }
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        &self.module.print_to_stderr();
    }

    #[allow(dead_code)]
    pub fn emit_file(&self, filename: &str) {
        let _ = self.module.verify().map_err(|err| {
          panic!(err.to_string());
        });
        let _ = self.module.print_to_file(Path::new(filename)).map_err(|err| {
                  panic!(err.to_string());
                });
    }
}
