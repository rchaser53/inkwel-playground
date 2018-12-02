extern crate inkwell;

use std::path::Path;

use inkwell::context::Context;
use inkwell::targets::{InitializationConfig, Target};
use std::error::Error;

fn run() -> Result<(), Box<Error>> {
    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();

    let i64_type = context.i64_type();
    let bool_type = context.bool_type();
    let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);

    let parent = module.add_function("main", fn_type, None);
    let basic_block = context.append_basic_block(&parent, "entry");
    builder.position_at_end(&basic_block);

    let true_value = bool_type.const_int(1, false);

    // build branch
    let then_bb = context.append_basic_block(&parent, "then");
    let else_bb = context.append_basic_block(&parent, "else");
    let cont_bb = context.append_basic_block(&parent, "ifcont");

    builder.build_conditional_branch(true_value, &then_bb, &else_bb);

    builder.position_at_end(&then_bb);
    let then_val = i64_type.const_int(1, false);
    builder.build_unconditional_branch(&cont_bb);

    let then_bb = builder.get_insert_block().unwrap();
    builder.position_at_end(&else_bb);
    let else_val = i64_type.const_int(2, false);
    builder.build_unconditional_branch(&cont_bb);

    builder.position_at_end(&cont_bb);

    let phi = builder.build_phi(context.i64_type(), "iftmp");

    phi.add_incoming(&[(&then_val, &then_bb), (&else_val, &else_bb)]);

    let ret_val = phi.as_basic_value().into_int_value();

    builder.build_return(Some(&ret_val));

    module.print_to_file(Path::new("nyan.ll"))?;

    Ok(())
}

fn main() {
    Target::initialize_native(&InitializationConfig::default()).unwrap();
    run().unwrap();
}
