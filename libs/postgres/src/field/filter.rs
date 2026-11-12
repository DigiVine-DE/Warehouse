use super::db::FieldPersistent;
use diesel_ext::Keywords;

#[derive(Default, Debug, Clone)]
pub struct FieldFilter<'a> {
    pub field_name_filter: Keywords<'a>,
    pub fruit_name_filter: Keywords<'a>,
    pub fuzzy_keywords: Keywords<'a>,
}

impl<'a> FieldFilter<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_field_names(mut self, words: impl Into<Keywords<'a>>) -> Self {
        self.field_name_filter = words.into();
        self
    }

    #[must_use]
    pub fn with_fruit_names(mut self, words: impl Into<Keywords<'a>>) -> Self {
        self.fruit_name_filter = words.into();
        self
    }

    #[must_use]
    pub fn with_fuzzy(mut self, words: impl Into<Keywords<'a>>) -> Self {
        self.fuzzy_keywords = words.into();
        self
    }
}

impl<'a> FieldFilter<'a> {
    pub(super) fn fuzzy_score(&self, field: &FieldPersistent) -> usize {
        self.fuzzy_count_case_sensitive(field) + self.fuzzy_count_case_insensitive(field)
    }

    fn fuzzy_count_case_sensitive(&self, field: &FieldPersistent) -> usize {
        let field_name = field.name.as_deref().unwrap_or_default();
        let fruit_name = field.fruit_name.as_deref().unwrap_or_default();
        self.fuzzy_keywords
            .raw()
            .iter()
            .map(|fuzzy_seg| {
                field_name.contains(fuzzy_seg) as usize + fruit_name.contains(fuzzy_seg) as usize
            })
            .sum()
    }

    fn fuzzy_count_case_insensitive(&self, field: &FieldPersistent) -> usize {
        let field_name = field
            .name
            .as_ref()
            .map_or(String::new(), |name| name.to_lowercase());
        let fruit_name = field
            .fruit_name
            .as_ref()
            .map_or(String::new(), |name| name.to_lowercase());

        self.fuzzy_keywords
            .lowercase()
            .iter()
            .map(|fuzzy_seg| {
                field_name.contains(fuzzy_seg) as usize + fruit_name.contains(fuzzy_seg) as usize
            })
            .sum()
    }
}
