#[macro_export]
macro_rules! generate_optional_setter {
    ($name: ident, $typ: ty) => {
        pub fn $name(&mut self, $name: $typ) -> &mut Self {
            self.$name = Some($name);
            self
        }
    };
}