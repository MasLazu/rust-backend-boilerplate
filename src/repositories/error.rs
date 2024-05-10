pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
    RowNotFound,
    UniqueConstraintViolation,
    DatabaseError,
}
