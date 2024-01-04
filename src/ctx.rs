#[derive(Debug, Clone)]
pub struct Ctx {
    user_id: u64,
}

impl Ctx {
    pub fn new(id: u64) -> Self {
        Self { user_id: id }
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}
