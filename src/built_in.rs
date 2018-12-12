use inkwell::values::FunctionValue;
use inkwell::AddressSpace;

use crate::creator::LLVMCreator;

pub fn create_printf(lc: &LLVMCreator) -> FunctionValue {
    let void_type = lc.context.void_type();
    let int8_type = lc.context.i8_type();

    let first_param_type = int8_type.ptr_type(AddressSpace::Generic);
    let fn_type = void_type.fn_type(&[first_param_type.into()], false);

    lc.module.add_function("printf", fn_type, None)
}

pub fn create_strcmp(lc: &LLVMCreator) -> FunctionValue {
    // let void_type = lc.context.void_type();
    let int8_type = lc.context.i8_type();
    let param_type = int8_type.ptr_type(AddressSpace::Generic);

    let int32_type = lc.context.i32_type();
    let fn_type = int32_type.fn_type(&[param_type.into(), param_type.into()], false);

    lc.module.add_function("strcmp", fn_type, None)
}
