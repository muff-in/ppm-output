fn write_ppm(w:i32, h:i32, max_value:i32){

    println!("P3\n {} {}\n {}", w, h, max_value);

    for j in 0..h{
        for i in 0..w {
            let r = i as f32 / w as f32;
            let g = j as f32 / h as f32;
            let b = 0.2;

            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        
        }
    }
}

fn main(){

    let width = 200;
    let height = 100;

    let max_value = 255;

    write_ppm(width, height, max_value);
}