pub trait Named {
    fn name(&self) -> &str;
    
    fn has_name_with(&self, string: &str) -> bool {
	self.name().to_lowercase()
	    .contains(&string.to_lowercase())
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    struct NamedStruct {
	name: String
    }
    impl Named for NamedStruct {
        fn name(&self) -> &str { &self.name }
    }
    
    #[test]
    fn has_name_when_name_includes_given_string() {
	let named_instance = NamedStruct { name: "New project".to_string() };

	let has_name = named_instance.has_name_with("new");

	assert_eq!(has_name, true);
    }

    #[test]
    fn do_not_has_name_when_name_does_not_include_given_string() {
	let named_instance = NamedStruct { name: "New project".to_string() };

	let has_name = named_instance.has_name_with("abc");

	assert_eq!(has_name, false);
    }
}
