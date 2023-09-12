///
/// User
///
/// # Example
/// ```
/// use user::User;
///
/// let user = User::new(1);
/// assert_eq!(user.id(), 1);
/// ```
pub struct User {
    id: u64,
}

impl User {
    pub const fn new(id: u64) -> Self {
        Self { id }
    }
    pub fn id(&self) -> u64 {
        self.id
    }
    // This method is not implemented.
    pub fn is_admin(&self) -> bool {
        false
    }
}

pub struct Admin {
    id: u64,
}

impl Admin {
    pub const fn new(id: u64) -> Self {
        Self { id }
    }
    pub fn get_id(&self) -> u64 {
        self.id
    }
    pub fn set_id(&mut self, id: u64) {
        self.id = id;
    }
    pub const fn is_admin(&self) -> bool {
        true
    }
}

fn main() {}