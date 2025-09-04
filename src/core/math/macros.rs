macro_rules! impl_vec_like_conversion {
    ($from: ty, $to: ty) => {
        impl From<&$crate::core::math::VecLike<$from>> for $crate::core::math::VecLike<$to> {
            fn from(value: &$crate::core::math::VecLike<$from>) -> Self {
                $crate::core::math::VecLike::new(value.x, value.y, value.z)
            }
        }

        impl From<$crate::core::math::VecLike<$from>> for $crate::core::math::VecLike<$to> {
            fn from(value: $crate::core::math::VecLike<$from>) -> Self {
                (&value).into()
            }
        }
    };
}

macro_rules! define_tuple_conversion {
    () => {
        pub(crate) fn transform(&self, transformation: &Matrix) -> Self {
            (transformation * crate::core::math::tuple::Tuple4::from(self)).into()
        }
    };
}

pub(crate) use impl_vec_like_conversion;
pub(crate) use define_tuple_conversion;
