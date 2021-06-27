use crate::projects::named::Named;
use crate::projects::project_error::ProjectError;

pub trait ListWithNames<T: Named> {
    fn items(&self) -> std::slice::Iter<T>;

    fn find(&self, search_string: &str) -> Result<&T, ProjectError> {
	let found: Vec<&T> = self.items()
	    .filter(|item| item.has_name_with(search_string))
	    .collect();

	match found.len() {
	    1 => Ok(found.get(0).unwrap()),
	    0 => Err(ProjectError::new("No project found")),
	    _ => Err(ProjectError::new("More than one project found"))
	}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct NamedStruct {
	name: String
    }
    impl Named for NamedStruct {
        fn name(&self) -> &str { &self.name }
    }

    struct ListWithNamesStruct {
	items: Vec<NamedStruct>
    }
    impl ListWithNames<NamedStruct> for ListWithNamesStruct {
	fn items(&self) -> std::slice::Iter<NamedStruct> { self.items.iter() }
    }
    
    #[test]
    fn it_finds_one_project() {
	let list = ListWithNamesStruct {
	    items: vec![
		NamedStruct { name: "Old Project X".to_string() },
		NamedStruct { name: "New Project Y".to_string() }
	    ]
	};
	
	let found_items = list.find("new");
	
	assert_eq!(found_items.unwrap(), list.items.get(1).unwrap());
    }

    #[test]
    fn errors_when_no_project_is_found() {
	let list = ListWithNamesStruct {
	    items: vec![]
	};
	
	let found_items = list.find("new");
	
	assert_eq!(found_items, Err(ProjectError::new("No project found")));
    }

    #[test]
    fn errors_when_more_than_one_project_is_found() {
	let list = ListWithNamesStruct {
	    items: vec![
		NamedStruct { name: "New Project Y".to_string() },
		NamedStruct { name: "Old Project X".to_string() },
		NamedStruct { name: "Another new Project Z".to_string() }
	    ]
	};
	
	let found_items = list.find("new");
	
	assert_eq!(found_items, Err(ProjectError::new("More than one project found")));
    }
}
