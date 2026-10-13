// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketTitle` type,
//   enforcing that the title is not empty and is not longer than 50 bytes.
//   Implement the traits required to make the tests pass too.

use crate::TicketDescription;

#[derive(Debug, thiserror::Error)]
pub enum ParseTitleError {
    #[error("The title cannot be empty")]
    TitleEmpty,
    #[error("The title cannot be longer than 50 bytes")]
    TitleLong,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TicketTitle(String);

impl TicketTitle {
    pub fn value(&self) -> &str { &self.0 }
}

impl TryFrom<String> for TicketTitle {
    type Error = ParseTitleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(ParseTitleError::TitleEmpty)
        }
        else if value.len() > 50 {
            Err(ParseTitleError::TitleLong)
        } else {
            Ok(TicketTitle(value))
        }
    }
}

impl TryFrom<&str> for TicketTitle {
    type Error = ParseTitleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}







