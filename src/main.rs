use child_care::state::{ChildManagement, NewBorn, Status, Condition};

fn main() {
    let mut system = ChildManagement::new();

    // Add a child
    let child = NewBorn {
        id: 1,
        name: "Jane".to_string(),
        status: Status::Alive,
        time_of_birth: "2023-07-10".to_string(),
        sex: "Female".to_string(),
        condition: Condition::Normal,
    };

    system.add_child(child);

    // Edit child details (partial update)
    system.edit_child_details(
        1,
        Some("Janet".to_string()),
        Some(Status::Still),
        Some("2023-07-11".to_string()),
        Some(Condition::Critical),
    );

    // OR: replace child entirely (full update)
    let updated = NewBorn {
        id: 1,
        name: "Janet Final".to_string(),
        status: Status::Still,
        time_of_birth: "2023-08-01".to_string(),
        sex: "Female".to_string(),
        condition: Condition::Critical,
    };

    match system.update_child(updated) {
        Ok(_) => println!("Child fully updated."),
        Err(e) => println!("Update error: {}", e),
    }

    // Print all children
    println!("\n--- All Children ---");
    for child in system.get_children() {
        println!(
            "ID: {}, Name: {}, Status: {:?}, DOB: {}, Sex: {}, Condition: {:?}",
            child.id, child.name, child.status, child.time_of_birth, child.sex, child.condition
        );
    }
}
