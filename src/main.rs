use koopa::back::KoopaGenerator;
use koopa::ir::builder_traits::*;
use koopa::ir::*;
use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::read_to_string;
use std::io::Result;

lalrpop_mod!(ox);

mod ast;

fn main() -> Result<()> {
    let mut args = args();
    args.next();
    let _mode = args.next().unwrap();
    let input = args.next().unwrap();
    args.next();
    let _output = args.next().unwrap();

    let input = read_to_string(input)?;
    let ast = ox::CompUnitParser::new().parse(&input).unwrap();

    println!("{:#?}", ast);

    let mut program = Program::new();

    let main_func = program.new_func_def(
        format!("@{}", ast.func_def.ident), // 1. 函数名 "@main"
        Vec::new(),                         // 2. 参数列表类型（无参数）
        Type::get_i32(),                    // 3. 返回值类型 i32
    );

    let func_data = program.func_mut(main_func);

    let entry_bb = func_data
        .dfg_mut()
        .new_bb()
        .basic_block(Some("%entry".into()));

    func_data
        .layout_mut()
        .bbs_mut()
        .push_key_back(entry_bb)
        .unwrap();

    let ret_val = func_data
        .dfg_mut()
        .new_value()
        .integer(ast.func_def.block.stmt.num);

    let ret = func_data.dfg_mut().new_value().ret(Some(ret_val));

    func_data
        .layout_mut()
        .bb_mut(entry_bb)
        .insts_mut()
        .push_key_back(ret)
        .unwrap();

    let mut generator = KoopaGenerator::from_path(_output)?;
    generator.generate_on(&program)?;

    Ok(())
}
