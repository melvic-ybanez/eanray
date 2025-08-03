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
