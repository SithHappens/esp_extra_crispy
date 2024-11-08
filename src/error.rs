pub type Result<'a, T> = core::result::Result<T, Error<'a>>;


#[derive(defmt::Format)]
pub enum Error<'a> {
    GenericError(&'a str),
}
