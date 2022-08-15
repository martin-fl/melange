use crate::interner;

use super::*;

use std::fmt::{self, Display, Formatter};

impl Display for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(interner::get(self.name))
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(path {}", self.segments[0])?;
        for segment in self.segments.iter().skip(1) {
            write!(f, " {}", segment)?;
        }
        write!(f, ")")
    }
}

impl Display for Tuple {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(tup {}", self.types[0])?;
        for ty in self.types.iter().skip(1) {
            write!(f, " {ty}")?;
        }
        write!(f, ")")
    }
}

impl Display for Array {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(array {} {})",
            self.ty,
            interner::get(self.len.symbol().unwrap())
        )
    }
}

impl Display for TyKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TyKind::Path(p) => write!(f, "{p}"),
            TyKind::Tuple(t) => write!(f, "{t}"),
            TyKind::Array(a) => write!(f, "{a}"),
        }
    }
}

impl Display for Ty {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl Display for Variant {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(variant {}", self.name)?;
        if !self.data.is_empty() {
            for ty in self.data.iter() {
                write!(f, " {ty}")?;
            }
        }
        write!(f, ")")
    }
}

impl Display for Enum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(enum")?;
        for variant in &self.variants {
            write!(f, " {variant}")?;
        }
        write!(f, ")")
    }
}

impl Display for Param {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(param {} {})", self.name, self.ty)
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(record")?;
        for field in &self.fields {
            write!(f, " {field}")?;
        }
        write!(f, ")")
    }
}

impl Display for TyDefKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TyDefKind::Alias(t) => write!(f, "{t}"),
            TyDefKind::Record(r) => write!(f, "{r}"),
            TyDefKind::Enum(e) => write!(f, "{e}"),
        }
    }
}

impl Display for TyDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(typedef {} {})", self.name, self.ty)
    }
}
