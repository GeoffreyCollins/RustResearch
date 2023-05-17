fn main() {
    let triangle = AreaOfTriangle{
        base: 5, height: 10
    };
    
    println!("The area of the triangle is: {:#?}", triangle.base * triangle.height);
}
#[derive(Debug)]
struct AreaOfTriangle
{
    base: u32,
    height: u32,
}