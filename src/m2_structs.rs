#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

// 値の読み取りのみであれば&mutを宣言する必要はない
impl User {
    fn increment_signin_count(&mut self){
        self.sign_in_count += 1
    }

    fn change_email(&mut self, new_email: &str){
        self.email = String::from(new_email)
    }

    fn is_active(&mut self){
        self.active = !self.active
    }
}

// どのようなユーザーであっても変更可能な参照を渡すことができる
// &mut - Pointer types, Stack
fn change_username(user: &mut User, new_username: &str){
    user.username = String::from(new_username)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_structs() {
        let mut user_1: User = User {
            username: String::from("someusername1"),
            email: String::from("someone@example.com"),
            active: true,
            sign_in_count: 1
        };
        
        change_username(&mut user_1,"somenewusername");

        dbg!(user_1);

        let mut user_2: User = User {
            username: String::from("someusername2"),
            email: String::from("someon2@example.com"),
            active: false,
            sign_in_count: 0
        };

        user_2.increment_signin_count();

        user_2.change_email("anotheremail.email.com");
        
        dbg!(user_2);
    }
}