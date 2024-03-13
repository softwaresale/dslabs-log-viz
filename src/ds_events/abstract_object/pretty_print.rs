use std::fmt::{Display, Formatter};
use crate::ds_events::abstract_object::{AbstractObject, AbstractValue};

pub struct AbstractObjectPrettyPrinter<'a> {
    /// the object to display
    obj: &'a AbstractObject,
    /// our current indent level
    indent_level: usize,
    /// true if start should indent
    should_indent_start: bool,
}

impl<'a> AbstractObjectPrettyPrinter<'a> {
    pub fn new(object: &'a AbstractObject) -> Self {
        Self {
            obj: object,
            indent_level: 0,
            should_indent_start: true,
        }
    }
    
    pub fn with_indent(mut self, indent: usize) -> Self {
        self.indent_level = indent;
        self
    }
    
    pub fn with_should_indent_start(mut self, status: bool) -> Self {
        self.should_indent_start = status;
        self
    }
    
    fn compute_base_indent_str(&self) -> String {
        (0..self.indent_level).map(|_| "  ").collect()
    }
    
    fn compute_prop_indent_str(&self) -> String {
        (0..(self.indent_level + 1)).map(|_| "  ").collect()
    }
}

impl<'a> Display for AbstractObjectPrettyPrinter<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let base_indent_str = self.compute_base_indent_str();
        let prop_indent_str = self.compute_prop_indent_str();
        if self.should_indent_start {
            write!(f, "{}", &base_indent_str, )?;
        }
        writeln!(f, "{}(", self.obj.name())?;
        for (name, prop) in self.obj.props() {
            write!(f, "{}{} = ", prop_indent_str, name)?;
            match prop {
                AbstractValue::Number(num) => write!(f, "{}", num),
                AbstractValue::Symbol(symb) => write!(f, "{}", symb),
                AbstractValue::Object(obj) => {
                    let obj_pp = Self::new(obj)
                        .with_indent(self.indent_level + 1)
                        .with_should_indent_start(false);
                    
                    write!(f, "{}", obj_pp)
                }
                AbstractValue::Map(map) => {
                    write!(f, "{:?}", map)
                }
                AbstractValue::Collection(collect) => {
                    write!(f, "{:?}", collect)
                }
            }?;
            write!(f, "\n")?;
        }
        write!(f, "{})", base_indent_str)
    }
}
