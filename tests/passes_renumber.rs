use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use vil::ir::basic_block::BasicBlock;
use vil::ir::function::Function;
use vil::ir::instruction::{Instruction, InstructionModifier, Opcode};
use vil::ir::module::Module;
use vil::ir::types::{Type, TypeKind};
use vil::optimizer::pass_manager::PassManager;
use vil::optimizer::passes::ssa_renumber::SSARenumber;

/// 构建一个简单的测试 IR 模块，包含一个函数和多个指令
fn build_test_module() -> Rc<RefCell<Module>> {
    // 创建模块
    let module = Rc::new(RefCell::new(Module::new("test_module".to_string())));
    
    // 创建函数
    let int_type = Type::get_int_type(TypeKind::Int32);
    let function = Rc::new(RefCell::new(Function::new(
        "test_function".to_string(),
        int_type.clone(),
        vec![],
    )));
    
    // 创建基本块
    let bb = Rc::new(RefCell::new(BasicBlock::new(
        "entry".to_string(),
        Some(function.clone()),
    )));
    
    // 添加指令
    let instr1 = Rc::new(RefCell::new(Instruction::new(
        Opcode::Add,
        int_type.clone(),
        InstructionModifier::None,
    )));
    instr1.borrow_mut().set_name("old_name_1".to_string());
    
    let instr2 = Rc::new(RefCell::new(Instruction::new(
        Opcode::Sub,
        int_type.clone(),
        InstructionModifier::None,
    )));
    instr2.borrow_mut().set_name("old_name_2".to_string());
    
    let instr3 = Rc::new(RefCell::new(Instruction::new(
        Opcode::Mul,
        int_type.clone(),
        InstructionModifier::None,
    )));
    instr3.borrow_mut().set_name("old_name_3".to_string());
    
    // 将指令添加到基本块
    bb.borrow_mut().add_instruction(instr1);
    bb.borrow_mut().add_instruction(instr2);
    bb.borrow_mut().add_instruction(instr3);
    
    // 将基本块添加到函数
    function.borrow_mut().add_basic_block(bb);
    
    // 将函数添加到模块
    module.borrow_mut().add_function(function);
    
    module
}

/// 检查指令名称是否按 SSA 格式重命名
fn check_ssa_names(module: &Rc<RefCell<Module>>) -> bool {
    let mut names = HashSet::new();
    let mut is_valid = true;
    
    for function in module.borrow().get_functions() {
        for bb in function.borrow().get_basic_blocks() {
            for instr in bb.borrow().get_instructions() {
                let instr_borrowed = instr.borrow();
                let name = instr_borrowed.get_name();
                
                // 检查名称格式是否为 %数字
                if !name.starts_with('%') {
                    is_valid = false;
                }
                
                // 检查名称是否唯一
                if !names.insert(name.to_string()) {
                    is_valid = false;
                }
            }
        }
    }
    
    is_valid
}

#[test]
fn test_ssa_renumber() {
    // 构建测试模块
    let module = build_test_module();
    
    // 运行 SSA Renumber Pass
    let mut pm = PassManager::new();
    pm.register_pass(SSARenumber);
    pm.add_to_pipeline("optimizer::SSARenumber");
    pm.run(&module).expect("PassManager 执行失败");
    
    // 验证结果
    assert!(check_ssa_names(&module));
    
    // 检查具体的指令名称
    let function = module.borrow().get_functions()[0].clone();
    let bb = function.borrow().get_basic_blocks()[0].clone();
    let bb_borrowed = bb.borrow();
    let instructions = bb_borrowed.get_instructions();
    
    assert_eq!(instructions[0].borrow().get_name(), "%0");
    assert_eq!(instructions[1].borrow().get_name(), "%1");
    assert_eq!(instructions[2].borrow().get_name(), "%2");
} 