use std::collections::HashMap;

pub struct VarType {
    name: String,
    size: usize,
}

pub enum Variable {
    Constant {
        var_type: VarType,
        exported: bool,
        data: Vec<u8>,
    },
    Function {
        var_type: VarType,
        exported: bool,
        params: Vec<VarType>,
        instructions: Vec<u8>,
    },
    Default {
        var_type: VarType,
        exported: bool,
        instructions: Vec<u8>,
    },
    Process {
        exported: bool,
        params: Vec<VarType>,
        instructions: Vec<u8>,
    },
}

impl Variable::Constant {
    pub fn new(exported: bool, var_type: VarType, data: Vec<u8>) -> Variable::Constant {
        Variable::Constant {
            var_type,
            exported,
            data,
        }
    }
}

impl Variable::Function {
    pub fn new(
        exported: bool,
        var_type: VarType,
        params: Vec<VarType>,
        instructions: Vec<u8>,
    ) -> Variable::Function {
        Variable::Function {
            var_type,
            exported,
            params,
            instructions,
        }
    }
}

impl Variable::Default {
    pub fn new(exported: bool, var_type: VarType, instructions: Vec<u8>) -> Variable::Default {
        Variable::Default {
            var_type,
            exported,
            instructions,
        }
    }
}

impl Variable::Process {
    pub fn new(exported: bool, params: Vec<VarType>, instructions: Vec<u8>) -> Variable::Process {
        Variable::Process {
            exported,
            params,
            instructions,
        }
    }
}

pub trait Context {
    fn def_var(&mut self, name: String, variable: Variable);
    fn get_type(&self, name: String) -> Option<VarType>;
}

//////////////////////////////////////////////////
//////////////////////////////////////////////////

pub struct FunctionContext<PCT: Context> {
    parent: PCT,
    variables: HashMap<String, Variable>,
}

impl<PCT: Context> FunctionContext<PCT> {
    pub fn new(parent: PCT) -> Self<PCT> {
        Self {
            parent,
            variables: HashMap::new(),
        }
    }
}

impl<PCT: Context> Context for FunctionContext<PCT> {
    fn def_var(&mut self, name: String, variable: Variable) {
        self.variables.insert(name, variable);
    }

    fn get_type(&self, name: String) -> Option<VarType> {
        if let Some(variable) = self.variables.get(&name) {
            match *variable {
                Variable::Constant(var_type, ..) => var_type,
                Variable::Function(var_type, ..) => var_type,
                Variable::Default(var_type) => var_type,
                Variable::Process => None,
            }
        } else if let Some(parent) = &self.parent {
            parent.get_type(name)
        } else {
            panic!("No defined variable of name {}", name)
        }
    }
}

///////////////////////////////////////////////////
///////////////////////////////////////////////////

pub struct ProcessContext<PCT: Context> {
    parent: PCT,
    variables: HashMap<String, Variable>,
}

impl<PCT: Context> ProcessContext<PCT> {
    pub fn new(parent: PCT) -> Self<PCT> {
        Self {
            parent,
            variables: HashMap::new(),
        }
    }
}

impl<PCT: Context> Context for ProcessContext<PCT> {
    fn def_var(&mut self, name: String, variable: Variable) {
        self.variables.insert(name, variable);
    }

    fn get_type(&self, name: String) -> Option<VarType> {
        if let Some(variable) = self.variables.get(&name) {
            match *variable {
                Variable::Constant(var_type, ..) => var_type,
                Variable::Function(var_type, ..) => var_type,
                Variable::Default(var_type) => var_type,
                Variable::Process => None,
            }
        } else if let Some(parent) = &self.parent {
            parent.get_type(name)
        } else {
            panic!("No defined variable of name {}", name)
        }
    }
}

//////////////////////////////////////////////////
//////////////////////////////////////////////////

pub struct SubContext<PCT: Context> {
    parent: PCT,
    contained: bool,
    variables: HashMap<String, Variable>,
}

impl<PCT: Context> SubContext<PCT> {
    pub fn new(parent: PCT) -> Self<PCT> {
        Self {
            parent,
            contained: false,
            variables: HashMap::new(),
        }
    }
}

impl<PCT: Context> Context for SubContext<PCT> {
    fn def_var(&mut self, name: String, variable: Variable) {
        self.variables.insert(name, variable);
    }

    fn get_type(&self, name: String) -> Option<VarType> {
        if let Some(variable) = self.variables.get(&name) {
            match *variable {
                Variable::Constant(var_type, ..) => var_type,
                Variable::Function(var_type, ..) => var_type,
                Variable::Default(var_type) => var_type,
                Variable::Process => None,
            }
        } else if let Some(parent) = &self.parent {
            parent.get_type(name)
        } else {
            panic!("No defined variable of name {}", name)
        }
    }
}

//////////////////////////////////////////////////
//////////////////////////////////////////////////

pub struct FileContext {
    variables: HashMap<String, Variable>,
}
impl FileContext {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
}

impl Context for FileContext {
    fn def_var(&mut self, name: String, variable: Variable) {
        self.variables.insert(name, variable);
    }

    fn get_type(&self, name: String) -> Option<VarType> {
        if let Some(variable) = self.variables.get(&name) {
            match *variable {
                Variable::Constant(var_type, ..) => var_type,
                Variable::Function(var_type, ..) => var_type,
                Variable::Default(var_type) => var_type,
                Variable::Process => None,
            }
        } else {
            panic!("No defined variable of name {}", name)
        }
    }
}
