extern crate inkwell;

use std::path::Path;
use std::error::Error;

mod creator;
use self::creator::*;

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
    let then_val = i64_type.const_int(1, false);
    lc.builder.build_unconditional_branch(&cont_bb);

    let then_bb = lc.builder.get_insert_block().unwrap();
    lc.builder.position_at_end(&else_bb);
    let else_val = i64_type.const_int(2, false);
    lc.builder.build_unconditional_branch(&cont_bb);

    lc.builder.position_at_end(&cont_bb);

    let phi = lc.builder.build_phi(lc.context.i64_type(), "iftmp");

    phi.add_incoming(&[(&then_val, &then_bb), (&else_val, &else_bb)]);

    let ret_val = phi.as_basic_value().into_int_value();

    lc.builder.build_return(Some(&ret_val));

    lc.module.print_to_file(Path::new("nyan.ll"))?;

    Ok(())
}

fn main() {
    run().unwrap();
}
