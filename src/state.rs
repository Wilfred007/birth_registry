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
   pub children: Vec<NewBorn>,
}