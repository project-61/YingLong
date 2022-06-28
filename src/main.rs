use std::collections::HashMap;

// /*
// use libyinglong::pass::gen_verilog;
use libyinglong::pass::gen_verilog::GenVerilog;
use libyinglong::pass::gen_verilog::GenVerilogEnv;
/*
use libyinglong::pass::type_infer::GlobalEnv;
use libyinglong::pass::type_infer::TypeCheck;
use libyinglong::pass::*;
//  */
use libyinglong::ylir::type_system::Type;
use libyinglong::ylir::type_system::TypeBind;
use libyinglong::ylir::*;

fn main() {
    let r = ("c".to_string(),
    Expr::Primop(
        Primop::Add,
        vec![Expr::Ref("a".to_string()), Expr::Ref("b".to_string())],
    ));
    let mut nodes = HashMap::new();
    nodes.insert(r.0, r.1);
    let r = Circuit {
        pos: None,
        id: "Circuit114514".to_string(),
        modules: vec![Module {pos:None,id:"add".to_string(),ports:vec![Port{pos:None,dir:Dir::Input,bind:TypeBind("a".to_string(),Type::Uint(32)),},Port{pos:None,dir:Dir::Input,bind:TypeBind("b".to_string(),Type::Uint(32)),},Port{pos:None,dir:Dir::Output,bind:TypeBind("c".to_string(),Type::Uint(32)),},],
        wire_defs: HashMap::new(), reg_defs: HashMap::new(), mem_defs: HashMap::new(), module_insts: HashMap::new(), nodes}],
    };

    // let pm = r.type_check(());
    // println!("analysis:\n{:?}", pm);

    // let pm = pm.unwrap();


    let out = r.gen_verilog(&GenVerilogEnv());

    println!("{}", out);
    //  */
}
