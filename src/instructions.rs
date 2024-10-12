macro_rules! def_instructions {
    ($name: ident ($param: expr) => $byte: literal) => {
        pub const $name: Instruction = Instruction { byte: $byte, param_type: $param };
    };
    ($name: ident ($param: expr) => $byte: literal, $($names:ident ($params: expr) => $bytes: literal),+) => {
        pub const $name: Instruction = Instruction { byte: $byte, param_type: $param };
        def_instructions!($($names ($params) => $bytes),+);
    };
}

pub struct Instruction {
    pub byte: u8,
    pub param_type: ParamType
}

pub enum ParamType {
    Nil,
    Static(u8),
    Variadic(u8)
}

def_instructions!(
    EXIT(ParamType::Nil) => 0x00,
    DEBUG(ParamType::Nil) => 0x01,
    GOTO(ParamType::Static(1)) => 0x02
);