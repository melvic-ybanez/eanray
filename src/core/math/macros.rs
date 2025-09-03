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

pub(crate) use impl_vec_like_conversion;
