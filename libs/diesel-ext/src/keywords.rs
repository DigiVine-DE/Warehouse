use once_cell::unsync::OnceCell;

/// `Keywords` is a container for a set of keywords, with private fields for different processed versions of the raw keywords, each of which is processed lazily using `OnceCell`
#[derive(Default, Debug, Clone)]
pub struct Keywords<'a> {
    raw: Vec<&'a str>,
    lowercase: OnceCell<Vec<String>>,
    like_pattern: OnceCell<Vec<String>>,
}

impl<'a> Keywords<'a> {
    /// Returns a reference to the original `raw` keywords.
    pub fn raw(&self) -> &[&str] {
        &self.raw
    }

    /// Returns a reference to the lowercase version of the `raw` keywords, using `OnceCell` for lazy evaluation.
    pub fn lowercase(&self) -> &[String] {
        self.lowercase
            .get_or_init(|| self.raw.iter().map(|x| x.to_lowercase()).collect())
    }

    /// Returns a reference to the processed keywords suitable for use in `LIKE`/`ILIKE` patterns in PostgreSQL, using `OnceCell` for lazy evaluation.
    pub fn like_pattern(&self) -> &[String] {
        self.like_pattern.get_or_init(|| {
            self.raw
                .iter()
                .map(|x| {
                    format!(
                        "%{}%",
                        x.replace(r#"\"#, r#"\\"#)
                            .replace('%', r#"\%"#)
                            .replace('_', r#"\_"#)
                    )
                })
                .collect()
        })
    }
}

impl<'a> FromIterator<&'a str> for Keywords<'a> {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        Self {
            raw: iter.into_iter().collect(),
            ..Default::default()
        }
    }
}

impl<'a, S: AsRef<str>> From<&'a Vec<S>> for Keywords<'a> {
    fn from(value: &'a Vec<S>) -> Self {
        value.iter().map(|x| x.as_ref()).collect()
    }
}

impl<'a, T: Into<Keywords<'a>>> From<Option<T>> for Keywords<'a> {
    fn from(value: Option<T>) -> Self {
        if let Some(v) = value {
            v.into()
        } else {
            Keywords::default()
        }
    }
}
