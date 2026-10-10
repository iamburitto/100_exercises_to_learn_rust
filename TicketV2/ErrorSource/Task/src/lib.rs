pub use crate::status::{ParseStatusError, Status};

// We've seen how to declare modules in one of the earliest exercises, but
// we haven't seen how to extract them into separate files.
// Let's fix that now!
//
// In the simplest case, when the extracted module is a single file, it is enough to
// create a new file with the same name as the module and move the module content there.
// The module file should be placed in the same directory as the file that declares the module.
// In this case, `src/lib.rs`, thus `status.rs` should be placed in the `src` directory.
mod status;

// TODO: Add a new error variant to `TicketNewError` for when the status string is invalid.
//   When calling `source` on an error of that variant, it should return a `ParseStatusError` rather than `None`.

// #[derive(thiserror::Error)]:
// this is the syntax to derive the Error trait for a custom error type, helped by thiserror.
// #[error("{0}")]:
// this is the syntax to define a Display implementation for each variant
//   of the custom error type. {0} is replaced by the zero-th field of the variant
//   (String, in this case) when the error is displayed.

// A field annotated with the #[from] attribute will automatically be used as the source
// of the error and thiserror will automatically generate a From implementation to
// convert the annotated type into your error type.
// use thiserror::Error;
//
// #[derive(Error, Debug)]
// pub enum MyError {
//     #[error("Failed to connect to the database")]
//     DatabaseError {
//         #[from]
//         inner: std::io::Error
//     }
// }


#[derive(Debug, thiserror::Error)]
pub enum TicketNewError {
    #[error("Title cannot be empty")]
    TitleCannotBeEmpty,
    #[error("Title cannot be longer than 50 bytes")]
    TitleTooLong,
    #[error("Description cannot be empty")]
    DescriptionCannotBeEmpty,
    #[error("Description cannot be longer than 500 bytes")]
    DescriptionTooLong,
    #[error("{0}")]
    InvalidStatus(#[from] ParseStatusError),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Result<Self, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleCannotBeEmpty);
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleTooLong);
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionCannotBeEmpty);
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionTooLong);
        }

        // TODO: Parse the status string into a `Status` enum.
       /* TODO */;

        // the ? operator will automatically convert the error type of the fallible operation
        // into the error type of the function, if a conversion is possible
        // (i.e. if there is a suitable From implementation)

        let status = Status::try_from(status)?;
        // let status = status.try_into()?; // also works



        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}
