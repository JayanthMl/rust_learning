#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
    
    let rect_using_tuples = (40, 60);
    
    println!(
        "The area of the rectangle is {} square pixels.",
        area_using_tuples(rect_using_tuples)
    );
    
    let rect_using_structs = Rectangle {
        width: 50,
        height: 70,
    };
    // println!("rect_using_structs is {rect_using_structs:?}");
    dbg!(&rect_using_structs);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_using_struct(&rect_using_structs)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_using_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_using_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}