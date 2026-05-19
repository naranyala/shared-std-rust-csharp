pub struct UserSession {
    pub id: u32,
    pub score: i32,
    pub username: String,
}

impl UserSession {
    pub fn new(id: u32, username: String) -> Self {
        Self {
            id,
            score: 0,
            username,
        }
    }

    pub fn add_score(&mut self, points: i32) {
        self.score += points;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_lifecycle() {
        let mut session = UserSession::new(1, "TestUser".to_string());
        assert_eq!(session.id, 1);
        assert_eq!(session.score, 0);
        
        session.add_score(10);
        assert_eq!(session.score, 10);
        
        session.add_score(-5);
        assert_eq!(session.score, 5);
    }
}
