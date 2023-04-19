use nalgebra::Vector3;


fn main() {

    let image_width = 256;
    let image_height = 256;

    println!("P3");
    

    println!("{} {}",image_width, image_height);
    println!("255");

/* 
    for _ in 0..163891{
        println!("255 0 0")
    }
*/


    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j + 1);
        for i in 0..image_width {

            let pixel_color = Vector3::new(i as f32 / (image_width-1) as f32, j as f32 / (image_height-1) as f32, 0.25);
            write_color(pixel_color);
            /* 
            let r = i as f64 / (image_width-1) as f64;
            let g = j as f64 / (image_height-1) as f64;
            let b: f64 = 0.25;
            */

        }
        
        
    }

    fn write_color(color: Vector3<f64>) {

        let ir = (255.999 * color.x) as i32;
        let ig = (255.999 * color.y) as i32;
        let ib = (255.999 * color.z) as i32;

        println!("{ir} {ig} {ib}")

    }

    //println!("Hello, world!");
}
