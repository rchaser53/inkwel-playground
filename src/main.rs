extern crate inkwell;

use inkwell::AddressSpace;
use inkwell::module::Module;
use inkwell::values::GlobalValue;
use inkwell::types::BasicType;

use std::error::Error;

mod creator;
use self::creator::*;

mod built_in;
use self::built_in::*;

fn run() -> Result<(), Box<Error>> {
    let lc = LLVMCreator::new("main");

    let i64_type = lc.context.i64_type();
    let bool_type = lc.context.bool_type();
    let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);

    let parent = lc.module.add_function("main", fn_type, None);
    let basic_block = lc.context.append_basic_block(&parent, "entry");
    lc.builder.position_at_end(&basic_block);

    let true_value = bool_type.const_int(1, false);

    // build branch
    let then_bb = lc.context.append_basic_block(&parent, "then");
    let else_bb = lc.context.append_basic_block(&parent, "else");
    let cont_bb = lc.context.append_basic_block(&parent, "ifcont");

    lc.builder
        .build_conditional_branch(true_value, &then_bb, &else_bb);

    lc.builder.position_at_end(&then_bb);
    let then_val = lc.builder.build_int_add(
        i64_type.const_int(1, false),
        i64_type.const_int(2, false),
        "",
    );
    lc.builder.build_unconditional_branch(&cont_bb);

    let then_bb = lc.builder.get_insert_block().unwrap();
    lc.builder.position_at_end(&else_bb);
    let else_val = i64_type.const_int(2, false);
    lc.builder.build_unconditional_branch(&cont_bb);

    lc.builder.position_at_end(&cont_bb);

    let phi = lc.builder.build_phi(lc.context.i64_type(), "iftmp");

    phi.add_incoming(&[(&then_val, &then_bb), (&else_val, &else_bb)]);

    let string = lc.context.const_string("my_string", true);
    // println!("{}", string.print_to_string().to_string());
    let string_ref = lc.builder.build_alloca(string.get_type(), "test");
    lc.builder.build_store(string_ref, string);

    
    let i64_array_type = i64_type.array_type(3);
    // zeroinitializer
    let i64_array_val = i64_array_type.const_zero();
    let i64_array_ref = lc.builder.build_alloca(i64_array_type, "test_array");
    lc.builder.build_store(i64_array_ref, i64_array_val);
    // let i64_array_array = i64_array_type.const_array(&[i64_array_val, i64_array_val]);

    add_global(&lc.module, i64_type, "nekodesu");

    let ret_val = phi.as_basic_value().into_int_value();

    lc.builder.build_return(Some(&ret_val));

    create_printf(&lc);
    create_strcmp(&lc);

    lc.dump();

    Ok(())
}

pub fn add_global<T: BasicType>(module: &Module, llvm_type: T, name: &str) -> GlobalValue {
    module.add_global(llvm_type, Some(AddressSpace::Const), name)
}

fn main() {
    run().unwrap();
}
