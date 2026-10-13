// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketTitle` type,
//   enforcing that the title is not empty and is not longer than 50 bytes.
//   Implement the traits required to make the tests pass too.

use crate::TicketDescription;

#[derive(Clone, Debug, PartialEq)]
pub struct TicketTitle(String);

impl TicketTitle {
    pub fn value(&self) -> &str { &self.0 }
}

#[derive(Debug, thiserror::Error)]
pub enum TicketTitleError {
    #[error("The title cannot be empty")]
    Empty,
    #[error("The title cannot be longer than 50 bytes")]
    TooLong,
}

impl TryFrom<String> for TicketTitle {
    type Error = TicketTitleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate(&value)?;
        Ok(TicketTitle(value))
    }
}

impl TryFrom<&str> for TicketTitle {
    type Error = TicketTitleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // Self::try_from(value.to_owned())
        validate(value)?;
        Ok(Self(value.to_owned()))
    }
}

fn validate(title: &str) -> Result<(), TicketTitleError> {
    if title.is_empty() {
        Err(TicketTitleError::Empty)
    }
    else if title.len() > 50 {
        Err(TicketTitleError::TooLong)
    } else {
        Ok(())
    }
}








