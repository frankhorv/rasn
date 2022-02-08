use crate::types::Integer;

pub struct Constraints(&'static [Constraint]);

impl Constraints {
    pub const fn any(&self, any: impl Fn(&Constraint) -> bool) -> bool {
        while let Some(i) = (0..self.0.len()).next() {
            if (any)(&self.0[i]) {
                return true
            }
        }

        false
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    Value {
        start: Option<Integer>,
        end: Option<Integer>
    },

    Size {
        min: Option<usize>,
        max: Option<usize>,
    },

    Extensible,
}

impl Constraint {
    /// Returns whether the type is extensible.
    pub const fn is_extensible(&self) -> bool {
        matches!(self, Self::Extensible)
    }
}
