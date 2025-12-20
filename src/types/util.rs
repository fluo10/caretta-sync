use std::path::Path;

#[cfg(feature = "server")]
pub(crate) fn path_to_sqlite_connect_options<P>(path : &P) -> sea_orm::ConnectOptions 
where 
    P: AsRef<Path>
{
    format!("sqlite://{}?mode=rwc", path.as_ref().to_string_lossy()).into()

}