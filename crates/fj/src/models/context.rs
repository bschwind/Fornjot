use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use crate::models::Error;

/// Contextual information passed to a [`Model`][crate::models::Model] when it
/// is being initialized.
///
/// Check out the [`ContextExt`] trait for some helper methods.
pub trait Context {
    /// Get an argument that was passed to this model.
    fn get_argument(&self, name: &str) -> Option<&str>;
}

impl<C: Context + ?Sized> Context for &'_ C {
    fn get_argument(&self, name: &str) -> Option<&str> {
        (**self).get_argument(name)
    }
}

impl<C: Context + ?Sized> Context for Box<C> {
    fn get_argument(&self, name: &str) -> Option<&str> {
        (**self).get_argument(name)
    }
}

impl<C: Context + ?Sized> Context for std::rc::Rc<C> {
    fn get_argument(&self, name: &str) -> Option<&str> {
        (**self).get_argument(name)
    }
}

impl<C: Context + ?Sized> Context for std::sync::Arc<C> {
    fn get_argument(&self, name: &str) -> Option<&str> {
        (**self).get_argument(name)
    }
}

impl Context for HashMap<String, String> {
    fn get_argument(&self, name: &str) -> Option<&str> {
        self.get(name).map(|s| s.as_str())
    }
}

/// Extension methods for the [`Context`] type.
///
/// By splitting these methods out into a separate trait, [`Context`] can stay
/// object-safe while allowing convenience methods that use generics.
pub trait ContextExt {
    /// Get an argument, returning a [`MissingArgument`] error if it doesn't
    /// exist.
    fn get_required_argument(
        &self,
        name: &str,
    ) -> Result<&str, MissingArgument>;

    /// Parse an argument from its string representation using [`FromStr`].
    fn parse_argument<T>(&self, name: &str) -> Result<T, ContextError>
    where
        T: FromStr,
        T::Err: std::error::Error + Send + Sync + 'static;

    /// Try to parse an argument, if it is present.
    fn parse_optional_argument<T>(
        &self,
        name: &str,
    ) -> Result<Option<T>, ParseFailed>
    where
        T: FromStr,
        T::Err: std::error::Error + Send + Sync + 'static;
}

impl<C: Context + ?Sized> ContextExt for C {
    fn get_required_argument(
        &self,
        name: &str,
    ) -> Result<&str, MissingArgument> {
        self.get_argument(name).ok_or_else(|| MissingArgument {
            name: name.to_string(),
        })
    }

    fn parse_argument<T>(&self, name: &str) -> Result<T, ContextError>
    where
        T: FromStr,
        T::Err: std::error::Error + Send + Sync + 'static,
    {
        let value = self.get_required_argument(name)?;

        value
            .parse()
            .map_err(|e| ParseFailed {
                name: name.to_string(),
                value: value.to_string(),
                error: Box::new(e),
            })
            .map_err(ContextError::from)
    }

    fn parse_optional_argument<T>(
        &self,
        name: &str,
    ) -> Result<Option<T>, ParseFailed>
    where
        T: FromStr,
        T::Err: std::error::Error + Send + Sync + 'static,
    {
        let value = match self.get_argument(name) {
            Some(value) => value,
            None => return Ok(None),
        };

        let parsed = value.parse().map_err(|e| ParseFailed {
            name: name.to_string(),
            value: value.to_string(),
            error: Box::new(e),
        })?;

        Ok(Some(parsed))
    }
}

/// An error that may be returned from a [`Context`] method.
#[derive(Debug)]
pub enum ContextError {
    /// An argument was missing.
    MissingArgument(MissingArgument),
    /// An argument was present, but we were unable to parse it into the final
    /// type.
    ParseFailed(ParseFailed),
}

impl From<MissingArgument> for ContextError {
    fn from(m: MissingArgument) -> Self {
        ContextError::MissingArgument(m)
    }
}

impl From<ParseFailed> for ContextError {
    fn from(p: ParseFailed) -> Self {
        ContextError::ParseFailed(p)
    }
}

impl Display for ContextError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ContextError::MissingArgument(_) => {
                write!(f, "An argument was missing")
            }
            ContextError::ParseFailed(_) => {
                write!(f, "Unable to parse an argument")
            }
        }
    }
}

impl std::error::Error for ContextError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ContextError::MissingArgument(m) => Some(m),
            ContextError::ParseFailed(p) => Some(p),
        }
    }
}

/// The error returned when a required argument wasn't provided.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissingArgument {
    /// The argument's name.
    pub name: String,
}

impl Display for MissingArgument {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let MissingArgument { name } = self;

        write!(f, "The \"{name}\" argument was missing")
    }
}

impl std::error::Error for MissingArgument {}

/// The error returned when [`ContextExt::parse_argument()`] is unable to parse
/// the argument's value.
#[derive(Debug)]
pub struct ParseFailed {
    /// The argument's name.
    pub name: String,
    /// The actual value.
    pub value: String,
    /// The error that occurred.
    pub error: Error,
}

impl Display for ParseFailed {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let ParseFailed { name, value, .. } = self;

        write!(f, "Unable to parse the \"{name}\" argument (\"{value:?}\")")
    }
}

impl std::error::Error for ParseFailed {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&*self.error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_is_object_safe() {
        let _: &dyn Context;
    }
}
