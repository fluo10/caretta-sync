macro_rules! database_impl {
    ($SelfT:ty) => {
        impl AsRef<$SelfT> for $SelfT {
            fn as_ref(&self) -> &Self {
                self
            }
        }
        impl AsRef<sea_orm::DatabaseConnection> for $SelfT {
            fn as_ref(&self) -> &sea_orm::DatabaseConnection {
                &self.0
            }
        }
        impl From<sea_orm::DatabaseConnection> for $SelfT {
            fn from(value: sea_orm::DatabaseConnection) -> Self {
                Self(value)
            }
        }

        impl From<$SelfT> for sea_orm::DatabaseConnection {
            fn from(value: $SelfT) -> Self {
                value.0
            }
        }
    };
}

