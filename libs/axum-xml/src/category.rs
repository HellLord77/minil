use quick_xml::{DeError, Error};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(super) enum Category {
    Io,
    Syntax,
    Data,
    Eof,
}

impl Category {
    pub(super) fn classify(err: &DeError) -> Category {
        match err {
            DeError::InvalidXml(err) => match err {
                Error::Io(_) => Category::Io,
                _ => Category::Syntax,
            },
            DeError::Custom(_) | DeError::KeyNotRead | DeError::UnexpectedStart(_) => {
                Category::Data
            }
            DeError::UnexpectedEof => Category::Eof,
        }
    }
}
