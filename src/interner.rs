use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Symbol(u32);

#[derive(Default, Clone, Debug)]
pub struct Interner {
    syms: HashMap<String, Symbol>,
    strings: Vec<String>,
}

impl Interner {
    pub fn new() -> Self {
        Self {
            syms: HashMap::new(),
            strings: Vec::new(),
        }
    }

    pub fn insert(&mut self, string: impl Into<String>) -> Symbol {
        let string = string.into();
        if let Some(&sym) = self.syms.get(string.as_str()) {
            return sym;
        } else {
            let sym = Symbol(u32::try_from(self.strings.len()).unwrap());
            self.syms.insert(string.clone(), sym);
            self.strings.push(string);
            return sym;
        }
    }

    pub fn get(&self, sym: Symbol) -> &str {
        self.strings[usize::try_from(sym.0).unwrap()].as_str()
    }
}

pub static mut INTERNER: Option<Interner> = None;

pub fn intern(string: impl Into<String>) -> Symbol {
    match unsafe { &mut INTERNER } {
        Some(interner) => interner.insert(string),
        None => {
            unsafe {
                INTERNER = Some(Interner::new());
            }
            intern(string)
        }
    }
}

pub fn get(sym: Symbol) -> &'static str {
    match unsafe { &INTERNER } {
        Some(interner) => interner.get(sym),
        None => panic!("interner not initialized"),
    }
}
