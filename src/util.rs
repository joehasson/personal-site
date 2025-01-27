pub fn e500<T>(e: T) -> actix_web::Error
where
    T: 'static + std::fmt::Debug + std::fmt::Display,
{
    actix_web::error::ErrorInternalServerError(e)
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

pub fn read_env_or_panic(varname: &str) -> String {
    std::env::var(varname).unwrap_or_else(|_| panic!("Failed to read env var {}", varname))
}
