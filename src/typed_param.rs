use trillium::Conn;
use trillium_router::RouterConnExt;

pub trait TypedParamConnExt {
    fn typed_param<'a, T>(&'a self, param: &str) -> Option<T>
    where
        T: TryFrom<&'a str>;
}

impl TypedParamConnExt for Conn {
    fn typed_param<'a, T>(&'a self, param: &str) -> Option<T>
    where
        T: TryFrom<&'a str>,
    {
        if let Some(value) = self.param(param) {
            if let Ok(value) = T::try_from(value) {
                return Some(value);
            }
        }

        None
    }
}
