struct User{
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64,
}
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);
struct AlwaysEqual; //Unit Struct
struct Rectangle{
    width:u32,
    height:u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
//each struct can have multiple impl blocks
fn main() {
    let user1=User{
        email:String::from("someone@example.com"),
        username:String::from("someone@username.com");
        active:true,
        sign_in_count:1,
    };
    let user2=User{
        email:String::from("nobody@example.com"),
        ..user1
    };
    user1.active=false;
    let black=Color(0,0,0);
    let origin=Point(0,0,0);
    let rect1=Rectangle{width:30,height:50};
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
fn build_user(email:String,username:String){
    User{
        email,
        username,
        active:true,
        sign_in_count:1,
    }
}
