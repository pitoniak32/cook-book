use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Ctx {
    user_id: Option<u64>,
    req_uuid: Uuid,
}

impl Ctx {
    pub fn new(id: Option<u64>) -> Self {
        Self {
            user_id: id,
            req_uuid: Uuid::new_v4(),
        }
    }

    pub fn user_id(&self) -> Option<u64> {
        self.user_id
    }

    pub fn req_uuid(&self) -> Uuid {
        self.req_uuid
    }
}
