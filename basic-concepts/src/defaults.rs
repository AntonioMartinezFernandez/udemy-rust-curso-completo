pub fn defaults() {
    /*^TRAIT DEFAULT AND ENUM DEFAULT */
    let mut user = User::default();
    println!("{:?}", user);

    user._role = Role::ADMIN;
    println!("{:?}", user);
}

#[derive(Default, Debug)]
struct User {
    _name: String,
    _email: String,
    _active: bool,
    _role: Role,
}

#[derive(Default, Debug)]
enum Role {
    #[default]
    USER,
    ADMIN,
}
