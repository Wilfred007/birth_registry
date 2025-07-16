#[derive(Debug, PartialEq)]
pub enum Status {
    Alive,
    Still,
}

#[derive(Debug, PartialEq)]
pub enum Condition {
    Normal,
    Critical,
}

#[derive(Debug, PartialEq)]
pub struct NewBorn {
    pub id: u32,
    pub name: String,
    pub status: Status,
    pub time_of_birth: String,
    pub sex: String,
    pub condition: Condition,
}

pub struct ChildManagement {
    children: Vec<NewBorn>,
}

impl ChildManagement {
    // Initialize a new child management system
    pub fn new() -> Self {
        ChildManagement {
            children: Vec::new(),
        }
    }

    // Add a new child to the system
    pub fn add_child(&mut self, child: NewBorn) {
        self.children.push(child);
    }

    // Edit a child's details by ID
    pub fn edit_child_details(
        &mut self,
        id: u32,
        name: Option<String>,
        status: Option<Status>,
        time_of_birth: Option<String>,
        condition: Option<Condition>,
    ) {
        if let Some(child) = self.children.iter_mut().find(|c| c.id == id) {
            if let Some(new_name) = name {
                child.name = new_name;
            }
            if let Some(new_status) = status {
                child.status = new_status;
            }
            if let Some(new_time_of_birth) = time_of_birth {
                child.time_of_birth = new_time_of_birth;
            }
            if let Some(new_condition) = condition {
                child.condition = new_condition;
            }
        } else {
            println!("Child with ID {} not found.", id);
        }
    }

    pub fn get_children(&self) -> &Vec<NewBorn> {
        &self.children
    }

    pub fn update_child(&mut self, updated_child: NewBorn) -> Result<(), String> {
        if let Some(existing) = self.children.iter_mut().find(|c| c.id == updated_child.id) {
            *existing = updated_child;
            Ok(())
        } else {
            Err(format!("Child with ID {} not found.", updated_child.id))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_edit_child() {
        let mut cm = ChildManagement::new();

        let child = NewBorn {
            id: 1,
            name: "Jane".to_string(),
            status: Status::Alive,
            time_of_birth: "2023-07-10".to_string(),
            sex: "Female".to_string(),
            condition: Condition::Normal,
        };

        cm.add_child(child);

        // Edit the child
        cm.edit_child_details(
            1,
            Some("Janet".to_string()),
            Some(Status::Still),
            Some("2023-07-11".to_string()),
            Some(Condition::Critical),
        );

        // Get the edited child
        let updated_child = cm.get_children().iter().find(|c| c.id == 1).unwrap();

        assert_eq!(updated_child.name, "Janet");
        assert_eq!(updated_child.status, Status::Still);
        assert_eq!(updated_child.time_of_birth, "2023-07-11");
        assert_eq!(updated_child.condition, Condition::Critical);
    }

    #[test]
    fn test_edit_non_existent_child() {
        let mut cm = ChildManagement::new();
        cm.edit_child_details(
            42,
            Some("Ghost".to_string()),
            Some(Status::Still),
            Some("1900-01-01".to_string()),
            Some(Condition::Critical),
        );

        // Should remain empty
        assert!(cm.get_children().is_empty());
    }


    #[test]
fn test_update_child() {
    let mut cm = ChildManagement::new();

    let child = NewBorn {
        id: 1,
        name: "Jane".to_string(),
        status: Status::Alive,
        time_of_birth: "2023-07-10".to_string(),
        sex: "Female".to_string(),
        condition: Condition::Normal,
    };

    cm.add_child(child);

    let updated_child = NewBorn {
        id: 1, // same ID to match
        name: "Janet Updated".to_string(),
        status: Status::Still,
        time_of_birth: "2023-08-01".to_string(),
        sex: "Female".to_string(),
        condition: Condition::Critical,
    };

    let result = cm.update_child(updated_child);
    assert!(result.is_ok());

    let retrieved = cm.get_children().iter().find(|c| c.id == 1).unwrap();
    assert_eq!(retrieved.name, "Janet Updated");
    assert_eq!(retrieved.status, Status::Still);
    assert_eq!(retrieved.condition, Condition::Critical);
}

}

