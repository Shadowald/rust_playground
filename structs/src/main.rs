struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main() {
    let mut user1 = User{
        email: String::from("fnhf13@gmail.com"),
        username: String::from("fnhf13"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    user1.username = String::from("Boldenburg");

    let user2 = build_user(
        String::from("an_email@realplace.com"), 
        String::from("NotABot")
    );

    let user3 = User {
        email: String::from("spam@fakeaccount.com"),
        username: String::from("RealGurl58723"),
        ..user2
    };


    // tuple structs
    struct Color(i32, i32, i32);

    struct Point(i32, i32, i32);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
