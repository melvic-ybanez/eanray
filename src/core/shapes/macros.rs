macro_rules! define_with_transform {
    () => {
        pub(crate) fn with_transform(&self, transform: $crate::core::math::Matrix) -> Self {
            use $crate::core::math::Transform;

            let transform: Transform = Transform::from_forward(transform);

            Self {
                fields: self.fields.with_transform(transform),
                ..self.clone()
            }
        }
    };
}

pub(crate) use define_with_transform;