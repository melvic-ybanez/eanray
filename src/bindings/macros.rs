macro_rules! from_user_data {
    ($name: ident, $t: ty) => {
        $name.borrow::<$t>()?.clone()
    };
}

pub(in crate::bindings) use from_user_data;