#[derive(PartialEq, Debug)]
struct Rectangle{
    width : u32,
    height : u32,
}

impl Rectangle {

    fn new(width : u32 , height : u32) -> Rectangle{
        Rectangle{
            width,
            height,
        }
    }

    pub fn can_hold(&self , other : &Rectangle) -> bool{
        if self.width >= other.width && self.height >= other.height {
            true
        }
        else{
            false
        }
    }
}

#[cfg(test)] // specifies that the following module is Test Module
mod tests {
    use super::*; // since the funtions inside this module cannot access functions outside this crate , we have to use `super`

    #[test]
    fn larger_can_hold_smaller(){
        let r1 = Rectangle::new(10,9);
        let r2 = Rectangle::new(6,9);
        assert!(r1.can_hold(&r2));
    }
}