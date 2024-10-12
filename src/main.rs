use crate::context::{Context, FileContext, Variable};
use crate::vm::{ByteStack, Program, VirtualMachine};

pub mod instructions;
mod vm;
mod compiler;
mod context;

fn main() {
    let file = FileContext::new();
    file.def_var("Hello", Variable::Constant::new());
    let program = Program {
        file
    };

    println!("{:?}", program)

    // let mut virtual_machine: VirtualMachine<1024> = VirtualMachine::new();
    // virtual_machine.write_array(0, &[1, 1, 2, 1]);
    // virtual_machine.exec();
}
