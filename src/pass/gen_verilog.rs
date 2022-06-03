use std::borrow::Borrow;
use std::fmt::format;

use rayon::prelude::*;

use crate::ylir::*;
use crate::ylir::type_system::*;
use super::PurePass;


pub struct GenVerilog();


impl PurePass<GenVerilog> for Circuit {
    type Target = String;

    fn pure_pass(&self, _pm: &GenVerilog) -> Self::Target {
        self.modules
            .par_iter()
            .map(|i| i.pure_pass(&GenVerilog()))
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

impl PurePass<GenVerilog> for Module {
    type Target = String;

    fn pure_pass(&self, pm: &GenVerilog) -> Self::Target {
        format!(
            "module {}(\n{}\n);\n{}endmodule;",
            self.id,
            self.ports.pure_pass(pm),
            self.stmts.pure_pass(pm)
        )
    }
}


impl PurePass<GenVerilog> for Ports {
    type Target = String;

    fn pure_pass(&self, pm: &GenVerilog) -> Self::Target {
        self.par_iter()
            .map(|p| p.pure_pass(pm))
            // .collect()
            .collect::<Vec<_>>().join(",\n")
    }
}


impl PurePass<GenVerilog> for Port {
    type Target = String;

    fn pure_pass(&self, pm: &GenVerilog) -> Self::Target {
        let dir = match self.dir {
            Dir::Input => "input",
            Dir::Output => "output",
        };
        format!("\t{} {}", dir, self.bind.pure_pass(pm))
    }
}


impl PurePass<GenVerilog> for StmtGroup {
    type Target = String;

    fn pure_pass(&self, pm: &GenVerilog) -> Self::Target {
        self.0.par_iter()
            .map(|s| s.pure_pass(pm))
            .collect()
            // .collect::<Vec<_>>().join("\n")
    }
}


impl PurePass<GenVerilog> for Stmt {
    type Target = String;

    fn pure_pass(&self, pm: &GenVerilog) -> Self::Target {
        format!("{}\t{}\n", self.raw_stmt.pure_pass(pm), self.pos.pure_pass(pm))
    }
}


impl PurePass<GenVerilog> for RawStmt {
    type Target = String;

    fn pure_pass(&self, pm: &GenVerilog) -> Self::Target {
        match self {
            RawStmt::WireDef(w) => w.pure_pass(pm),
            RawStmt::RegDef(bind, value, append) => todo!(),
            RawStmt::MemDef(memdef) => todo!(),
            RawStmt::Inst(name, value) => format!("\tassign {} = {};", name, value.pure_pass(pm)),
            RawStmt::Node(name, value) => format!("\tassign {} = {};", name, value.pure_pass(pm)),
            RawStmt::Connect(a, b) => todo!(),
            RawStmt::When(w) => w.pure_pass(pm),
            RawStmt::StmtGroup(sg) => sg.pure_pass(pm),
        }
    }
}


impl PurePass<GenVerilog> for Expr {
    type Target = String;

    fn pure_pass(&self, pm: &GenVerilog) -> Self::Target {
        match self {
            Expr::Literal(_) => todo!(),
            Expr::Ref(id) => id.clone(),
            Expr::SubField(_, _) => todo!(),
            Expr::SubIndex(_, _) => todo!(),
            Expr::SubAccess(_, _) => todo!(),
            Expr::Mux(cond, then, else_) => todo!(),
            // Expr::Validif(_, _) => todo!(),
            Expr::Primop(op, params) => match op {
                Primop::Add => format!("{} + {}", params[0].pure_pass(pm), params[1].pure_pass(pm)),
                Primop::Sub => format!("{} - {}", params[0].pure_pass(pm), params[1].pure_pass(pm)),
                Primop::Mul => format!("{} * {}", params[0].pure_pass(pm), params[1].pure_pass(pm)),
                Primop::Div => format!("{} / {}", params[0].pure_pass(pm), params[1].pure_pass(pm)),
                Primop::Mod => todo!(),
                Primop::Lt => todo!(),
                Primop::Leq => todo!(),
                Primop::Gt => todo!(),
                Primop::Geq => todo!(),
                Primop::Eq => todo!(),
                Primop::Neq => todo!(),
                Primop::Pad => todo!(),
                Primop::AsUInt => todo!(),
                Primop::AsSInt => todo!(),
                Primop::AsClock => todo!(),
                Primop::Shl => todo!(),
                Primop::Shr => todo!(),
                Primop::Dshl => todo!(),
                Primop::Dshr => todo!(),
                Primop::Cvt => todo!(),
                Primop::Neg => todo!(),
                Primop::Not => todo!(),
                Primop::And => todo!(),
                Primop::Or => todo!(),
                Primop::Xor => todo!(),
                Primop::Andr => todo!(),
                Primop::Orr => todo!(),
                Primop::Xorr => todo!(),
                Primop::Cat => todo!(),
                Primop::Bits => todo!(),
                Primop::Head => todo!(),
                Primop::Tail => todo!(),
            },
        }
    }
}

impl PurePass<GenVerilog> for When {
    type Target = String;

    fn pure_pass(&self, pm: &GenVerilog) -> Self::Target {
        if self.else_.is_none() {
            format!("\tif ({})\n\t\t{}\tend",
                self.cond.pure_pass(pm),
                self.then.pure_pass(pm)
            )
        } else {
            format!("\tif ({})\n\t\t{}\n\telse\n\t\t{}\n\tend",
                self.pure_pass(pm),
                self.then.pure_pass(pm),
                self.else_.as_ref().unwrap().pure_pass(pm)
            )
        }
    }
}


impl PurePass<GenVerilog> for TypeBind {
    type Target = String;

    fn pure_pass(&self, pm: &GenVerilog) -> Self::Target {
        let size = self.1.get_width().expect("error: width is unknown");
        if size == 0 {
            println!("warning: width is 0");
            return "".to_string();
        } else if size == 1 {
            format!("{}", self.0)
        } else {
            format!("[{}:0]\t{}", size, self.0)
        }
    }
}

impl PurePass<GenVerilog> for PosInfoOpt {
    type Target = String;

    fn pure_pass(&self, pm: &GenVerilog) -> Self::Target {
        match self {
            PosInfoOpt::None => "".to_string(),
            PosInfoOpt::Some(pos) => pos.pure_pass(pm),
        }
    }
}

impl PurePass<GenVerilog> for PosInfo {
    type Target = String;

    fn pure_pass(&self, pm: &GenVerilog) -> Self::Target {
        // fixme
        format!("@[\"{}\":{:?}:{:?}]", self.file, self.line, self.col)
    }
}