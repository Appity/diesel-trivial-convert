use std::fmt::{Display,Formatter,Result as DisplayResult};
use std::error::Error;

use diesel::{AsExpression,FromSqlRow};
use diesel::sql_types::Text;

#[derive(Clone,Debug,Eq,PartialEq)]
// Derive AsExpression and FromSqlRow (required)
#[derive(AsExpression,FromSqlRow)]
// Declare sql_type used (required)
#[diesel(sql_type=Text)]
pub struct Label {
    label: String
}

// Custom error for conversion issues (required)
#[derive(Debug)] // Debug for {:?} formatting (required)
pub enum LabelError {
    InvalidLabel
}

// Display to conform to Error trait (required)
impl Display for LabelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
        match self {
            Self::InvalidLabel => write!(f, "Invalid label")
        }
    }
}

// Error trait (required)
impl Error for LabelError { }

// Conversion from String (required)
impl TryFrom<String> for Label {
    type Error = LabelError;

    fn try_from(s: String) -> Result<Label, Self::Error> {
        if s.chars().all(char::is_alphanumeric) && s.len() > 0 {
            Ok(Label { label: s })
        }
        else {
            Err(LabelError::InvalidLabel)
        }
    }
}

// Conversion from &str, used for convenience (optional)
impl TryFrom<&str> for Label {
    type Error = LabelError;

    fn try_from(s: &str) -> Result<Label, Self::Error> {
        Ok(Label { label: s.into() })
    }
}

// Implement conversion from the type to &String (required)
impl<'a> TryFrom<&'a Label> for &'a String {
    type Error = LabelError;

    fn try_from(l: &'a Label) -> Result<&'a String, Self::Error> {
        Ok(&l.label)
    }
}

// Implement conversion from the type to String (optional)
impl TryFrom<Label> for String {
    type Error = LabelError;

    fn try_from(l: Label) -> Result<String, Self::Error> {
        Ok(l.label)
    }
}

// Implementation of the Postgres ToSql/FromSql
mod postgres {
    use super::*;

    use diesel::pg::Pg;

    impl diesel::serialize::ToSql<Text, Pg> for Label {
        fn to_sql(&self, out: &mut diesel::serialize::Output<'_, '_, Pg>) -> diesel::serialize::Result {
            <String as diesel::serialize::ToSql<Text, Pg>>::to_sql(self.try_into()?, &mut out.reborrow())
        }
    }

    impl diesel::deserialize::FromSql<Text, Pg> for Label {
        fn from_sql<'a>(bytes : diesel::backend::RawValue<'a,Pg>) -> diesel::deserialize::Result<Self> {
            <String as diesel::deserialize::FromSql<Text, Pg>>::from_sql(bytes)
                .and_then(|s| Self::try_from(s).map_err(|e| e.into()))
        }
    }
}
