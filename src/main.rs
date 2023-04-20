mod ray;


use nalgebra::Vector3;
use crate::ray::Ray;


fn main() {
    //image sizing
    let aspect: f32 = 16.0/9.0;
    let image_width = 400;
    let image_height = (image_width as f32/aspect) as i32;
    
    //camera options
    let view_height: f32 = 2.0;
    let view_width = view_height * aspect;
    let focal_length: f32 = 1.0;

    let origin = Vector3::new(0.0f32,0.0f32,0.0f32);
    let horizontal = Vector3::new(view_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, view_height, 0.0);
    let lower_left = origin - horizontal/2f32 - vertical/2f32 - Vector3::new(0.0, 0.0, focal_length);
    
    //auto lower_left_corner = origin - horizontal/2 - vertical/2 - vec3(0, 0, focal_length);
    eprintln!("{}",image_height);

    println!("P3");
    

    println!("{} {}",image_width, image_height);
    println!("255");



    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j + 1);
        for i in 0..image_width {

            let u = i as f32 / (image_width-1) as f32;
            let v = j as f32 / (image_height-1) as f32;
            let r = Ray::new(origin, lower_left + u*horizontal + v*vertical - origin);
            let pixel_color = ray_color(&r);



            /*ray r(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            color pixel_color = ray_color(r);
            write_color(std::cout, pixel_color); */

            //let pixel_color = Vector3::new(i as f32 / (image_width-1) as f32, j as f32 / (image_height-1) as f32, 0.25);
            write_color(pixel_color);
            /* 
            let r = i as f64 / (image_width-1) as f64;
            let g = j as f64 / (image_height-1) as f64;
            let b: f64 = 0.25;
            */

        }
        
        
    }


}

fn write_color(color: Vector3<f32>) {

    let ir = (255.999 * color.x) as i32;
    let ig = (255.999 * color.y) as i32;
    let ib = (255.999 * color.z) as i32;

    println!("{ir} {ig} {ib}")

}

fn ray_color(ray: &Ray) -> Vector3<f32> {
    let center = Vector3::new(0.0f32, 0.0f32, -1.0f32);
    if hit_sphere(center, 0.5f32, &ray) {
        return Vector3::new(1.0, 0.0, 0.0)
    }
    let unit_direction = ray.direction().normalize();
    let t = 0.5 * (unit_direction[1] + 1.0);
    (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: Vector3<f32>, radius: f32, ray: &Ray) -> bool {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(&ray.direction());
    let b = (oc.dot(&ray.direction())) * 2.0f32;
    let c = oc.dot(&oc) - radius.powf(2.0);
    let discriminant = b.powf(2.0) - ((a * c) * 4.0f32);
    return discriminant > 0.0f32

}

/*bool hit_sphere(const point3& center, double radius, const ray& r) {
    vec3 oc = r.origin() - center;
    auto a = dot(r.direction(), r.direction());
    auto b = 2.0 * dot(oc, r.direction());
    auto c = dot(oc, oc) - radius*radius;
    auto discriminant = b*b - 4*a*c;
    return (discriminant > 0);
    
    if (hit_sphere(point3(0,0,-1), 0.5, r))
        return color(1, 0, 0);
    
     */
