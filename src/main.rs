#[derive(Debug)]
struct Rectangle {
    width: i32,
    hight: i32,
}
//定义一个方法
impl Rectangle {
    fn area(&self) -> i32 {
        self.hight * self.width
    }
}

impl Rectangle {
    fn can_hold(&self,other: &Rectangle) -> bool {
        self.width > other.width && self.hight > other.hight
    }
}

fn area(rectangle: &Rectangle) -> i32{
    rectangle.hight * rectangle.width
}


fn main(){
    let rect1 = Rectangle {
        width: 10,
        hight: 20,
    };

    let s = area(&rect1);

    let s2 = &rect1.area();
    println!("{}",s);
    println!("{:?}",&rect1);

    println!("{}",s2);

    let rect2 = Rectangle {
        width: 30,
        hight: 40,
    };

    let b = &rect2.can_hold(&rect1);
    println!("{}",b);

}