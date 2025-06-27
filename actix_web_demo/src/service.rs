use crate::{model::{CreateUserDto, UpdateUserDto, User}, repository::UserRepository};

pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        UserService { repository }
    }

    pub fn create_user(&self, dto: CreateUserDto) -> User {
        // 实际项目中这里会有更复杂的业务逻辑
        let id = self.generate_next_id();
        let user = User {
            id,
            username: dto.username,
            email: dto.email,
        };
        self.repository.create(user)
    }

    pub fn get_all_users(&self) -> Vec<User> {
        self.repository.find_all()
    }

    pub fn get_user_by_id(&self, id: u32) -> Option<User> {
        self.repository.find_by_id(id)
    }

    pub fn update_user(&self, id: u32, dto: UpdateUserDto) -> Option<User> {
        self.repository.update(id, dto)
    }

    pub fn delete_user(&self, id: u32) -> bool {
        self.repository.delete(id)
    }

    fn generate_next_id(&self) -> u32 {
        let users = self.repository.find_all();
        users.iter().map(|u| u.id).max().unwrap_or(0) + 1
    }
}