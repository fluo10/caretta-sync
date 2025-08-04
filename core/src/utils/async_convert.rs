pub trait AsyncFrom<T> {
    async fn async_from(source: T) -> Self;
}
pub trait AsyncInto<T> {
    async fn async_into(self) -> T;
}
impl<T, U> AsyncInto<T> for U
where T: AsyncFrom<U> {
    async fn async_into(self) -> T {
        T::async_from(self).await
    }
}

pub trait AsyncTryFrom<T>: Sized {
    type Error: Sized;
    async fn async_try_from(source: T) -> Result<Self, Self::Error>;
}
pub trait AsyncTryInto<T>: Sized{
    type Error: Sized;
    async fn async_try_into(self) -> Result<T, Self::Error>;
}

impl<T, U> AsyncTryInto<T> for U
where T: AsyncTryFrom<U> {
    type Error = <T as AsyncTryFrom<U>>::Error;
    async fn async_try_into(self) -> Result<T, Self::Error> {
        T::async_try_from(self).await
    }
}