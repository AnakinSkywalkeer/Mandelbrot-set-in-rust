extern crate nannou;
use nannou::prelude::*;
use num_complex::Complex;
use float_lerp::lerp;
fn main() {
 nannou::sketch(view).run();
}

fn view(app: &App,frame: Frame){

    let draw=app.draw();
    draw.background().color(WHITE);
    let mut x=0.0;
    while x<1.0{
        let mut y=0.0;
        while y<1.0{
           let point_x=lerp(-2.0, 2.0, x);
           let point_y=lerp(-2.0, 2.0, y);
           let p=is_int_set(Complex::new(point_x,point_y)) as u32;
           if p==0
           {
               let f:f32=point_x as f32;
               let g:f32=point_y as f32;
               draw.ellipse()
               .color(nannou::color::rgb(71 as u64,66 as u64, 88 as u64))
               .w(4.0)
               .h(4.0)
               .x_y(f*300.0,g*300.0);
           }
           //else {
            //    let f:f32=point_x as f32;
             //  let g:f32=point_y as f32;
             //  draw.ellipse()
              // .color(WHITE)
              // .w(2.0)
             //  .h(2.0)
             //  .x_y(f*210.0,g*210.0);
          // }
           y+=0.001;
        }
        x+=0.001;

    }
    draw.to_frame(app,&frame).unwrap();
}
fn is_int_set(c:Complex<f64>)-> i32
{
    let mut z:Complex<f64>=Complex::new(0.0,0.0); 
    for i in 0..4000 {
        z=pow(z,2)+c;
        if z.norm()>16.0{
                return i;
        }
    }
    return 0;
}
