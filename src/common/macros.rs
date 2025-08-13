use std::ops::{Deref, DerefMut};

#[macro_export]
macro_rules! generate_optional_setter {
    ($obj: ident, $field: ident, $typ: ty) => {
        pub fn $field(&mut self, $field: $typ) -> &mut Self {
            self.$obj.$field = Some($field);
            self
        }
    };

    ($field: ident, $typ: ty) => {
        pub fn $field(&mut self, $field: $typ) -> &mut Self {
            self.$field = Some($field);
            self
        }
    };
}

#[macro_export]
macro_rules! impl_deref {
    ($deref_for: ty, $target: ty) => {
        impl Deref for $deref_for {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $deref_for {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}
