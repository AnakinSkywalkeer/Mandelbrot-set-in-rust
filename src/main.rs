extern crate nannou;
use nannou::prelude::*;

fn main() {
     nannou::app(model).update(update).run();
}
struct Model {
    _window: window::Id,
}
fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}
fn view(app: &App, _model: &Model, frame: Frame){

    let draw=app.draw();
    draw.background().color(WHITE);
    for x in 0..100  {
        for y in 0..100  {
           let mut a= map_range(x, 0, 100, -2, 2);
           let mut b= map_range(y, 0, 100, -2, 2);
           let mut n=0;
           while n<100 {
                let aa=a*a-b*b;
                let bb= 2*b*b;
                a=aa+a;
                b=bb+b;
                if a*a+b*b>16
                {
                    break;
                }
                n+=1;
           }
         // if the n becomes 100 that means points was not going towards infinity so we can plot
         // them
         let f=x as f32;
         let g=y as f32;
         let pix=1.5;
         if n==100
         {
           draw.ellipse()
               .color(RED)
               .w(pix)
               .h(pix)
               .x_y(f , g);
         }else {
           draw.ellipse()
               .color(WHITE)
               .w(pix)
               .h(pix)
               .x_y(f , g);    
         }
        }
    }
    draw.to_frame(app,&frame).unwrap();
}
