use uuid::Uuid;

pub struct Entity {
    pub uuid: Uuid,
    pub name: String,
}
impl Entity {
    pub fn create(name: &str) -> Self {
        Entity {
            uuid: Uuid::new_v4(),
            name: name.to_string(),
        }
    }
}

pub struct Player {

}
