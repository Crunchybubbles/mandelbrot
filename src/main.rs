use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;
use rand::Rng;



const SCREEN_WIDTH: usize = 1920;
const SCREEN_HEIGHT: usize = 1080;
// const COLORS: [[u8;4]; 10] = [[0x00,0x00,0x00,0xff], [0x00,0xff,0x00,0xff], [0xff, 0x00, 0x00, 0xff], [0xee, 0xfc, 0xa9, 0xff], [0x6a, 0x65, 0xa7, 0xff], [0xcf, 0x87, 0x72, 0xff], [0xa6, 0xe8, 0xf4, 0xff], [0xfc, 0x35, 0x13, 0xff], [0xff, 0xff, 0xff, 0xff], [0x00,0x00,0xff,0xff]];
// const COLORS: [[u8;4]; 2] = [[0x00,0x00,0x00,0xff], [0xff,0xff,0xff,0xff]];




fn main() -> Result<(), Error> {

    let colors: [[u8;4]; 10] = random_color_space();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (window, width, height, mut _hidpi_factor) = create_window("mandelbrot", &event_loop);
    let surface_texture = SurfaceTexture::new(width, height, &window);
    let mut pixels = Pixels::new(width as u32, height as u32, surface_texture)?;
    let  (xspace, yspace) = init_og_space();
    let mut spacearray: [([f64; SCREEN_WIDTH], [f64; SCREEN_HEIGHT]); 10] = [([0.0; SCREEN_WIDTH], [0.0; SCREEN_HEIGHT]); 10];
    let mut index = 0;
    let mut imagecount = 23;
    spacearray[0] = (xspace, yspace);

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            mandelbrot(&spacearray[index].0, &spacearray[index].1, pixels.get_frame(), &colors);
            if pixels
            .render()
            .map_err(|e| error!("pixels.render() failed: {}", e))
            .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }


        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }



            if input.key_pressed(VirtualKeyCode::Z) {
                // println!("old index: {}", index);
                let space = init_new_space(spacearray[index].0, spacearray[index].1, input.mouse().unwrap());
                if index == 9 {
                    index = 1;
                    spacearray[index] = space;

                } else {
                    index += 1;
                    // println!("new index: {}", index);
                    spacearray[index] = space;

                }
            }

            if input.key_pressed(VirtualKeyCode::X) {
                index -= 1;
            }

            if input.key_pressed(VirtualKeyCode::I) {
                let h: u32 = SCREEN_HEIGHT as u32;
                let w: u32 = SCREEN_WIDTH as u32;
                let name: String = format!("image{}.png",imagecount);
                image::save_buffer(name, pixels.get_frame(), w, h, image::ColorType::Rgba8).unwrap();
                imagecount += 1;
            }




            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            if let Some(factor) = input.scale_factor_changed() {
                _hidpi_factor = factor;
            }
            window.request_redraw();
        }
    });
}


fn init_og_space() -> ([f64; SCREEN_WIDTH], [f64; SCREEN_HEIGHT]) {

    let height: f64 = SCREEN_HEIGHT as f64;
    let width: f64 = SCREEN_WIDTH as f64;
    let x_min = -2.00;
    let x_max = 0.47;
    let y_min = -1.12;
    let y_max = 1.12;
    let mut xspace: [f64; SCREEN_WIDTH] = [0.0; SCREEN_WIDTH];
    let dx = (x_max - x_min) / width;
    for i in 0..SCREEN_WIDTH {
        if i == 0 {
            xspace[i] = x_min;
        } else if i == SCREEN_WIDTH - 1 {
            xspace[i] = x_max;
        } else {
            xspace[i] = xspace[i - 1] + dx;
        }
    }
    let mut yspace: [f64; SCREEN_HEIGHT] = [0.0; SCREEN_HEIGHT];
    let dy = (y_max - y_min) / height;
    for j in 0..SCREEN_HEIGHT {
        if j == 0 {
            yspace[j] = y_max;
        } else if j == SCREEN_HEIGHT - 1 {
            yspace[j] = y_min;
        } else {
            yspace[j] = yspace[j - 1] - dy;
        }
    }

    return (xspace, yspace);
}

fn init_new_space(oldxspace: [f64; SCREEN_WIDTH], oldyspace: [f64; SCREEN_HEIGHT], centerpixel: (f32, f32)) -> ([f64; SCREEN_WIDTH], [f64; SCREEN_HEIGHT]) {
    let height: f64 = SCREEN_HEIGHT as f64;
    let width: f64 = SCREEN_WIDTH as f64;
    let mousex: usize = centerpixel.0 as usize;
    let mousey: usize = centerpixel.1 as usize;
    let mut new_x_min: f64 = 0.0;
    let mut new_x_max: f64 = 0.0;
    let mut new_y_min: f64 = 0.0;
    let mut new_y_max: f64 = 0.0;

    if mousex < 48 {
        new_x_min = oldxspace[mousex];
        new_x_max = oldxspace[mousex + 96];
    } else if mousex > 1920 - 48 {
        new_x_max = oldxspace[mousex];
        new_x_min = oldxspace[mousex - 96];
    } else {
        new_x_min = oldxspace[mousex - 48];
        new_y_min = oldyspace[mousey + 27];
    }

    if mousey > 1080 - 27 {
        new_y_min = oldyspace[mousey];
        new_y_max = oldyspace[mousey - 54];

    } else if mousey < 27 {
        new_y_max = oldyspace[mousey];
        new_y_min = oldyspace[mousey + 54];

    } else {
        new_x_max = oldxspace[mousex + 48];
        new_y_max = oldyspace[mousey - 27];
    }
    println!("xmin:{} xmax:{} ymin:{} ymax:{}", new_x_min, new_x_max, new_y_min, new_y_max);
    println!("xrange:{} yrange:{}  zoom:{}", new_x_max-new_x_min, new_y_max-new_y_min, 2.47/(new_x_max-new_x_min));
    let mut new_x_space: [f64; SCREEN_WIDTH] = [0.0; SCREEN_WIDTH];
    let mut new_y_space: [f64; SCREEN_HEIGHT] = [0.0; SCREEN_HEIGHT];
    let dx = (new_x_max - new_x_min) / width;
    let dy = (new_y_max - new_y_min) / height;

    for i in 0..SCREEN_WIDTH {
        if i == 0 {
            new_x_space[i] = new_x_min;
        } else if i == SCREEN_WIDTH - 1 {
            new_x_space[i] = new_x_max;
        } else {
            new_x_space[i] = new_x_space[i - 1] + dx;
        }
    }

    for i in 0..SCREEN_HEIGHT {
        if i == 0 {
            new_y_space[i] = new_y_max;
        } else if i == SCREEN_HEIGHT - 1 {
            new_y_space[i] = new_y_min;
        } else {
            new_y_space[i] = new_y_space[i - 1] - dy;
        }
    }
    return (new_x_space, new_y_space);
}




fn mandelbrot(xspace: &[f64; SCREEN_WIDTH], yspace: &[f64; SCREEN_HEIGHT] ,screen: &mut [u8], colors: &[[u8; 4]; 10]) {
    clear(screen);

    let mut index = 0;
    for j in 0..SCREEN_HEIGHT {
        for i in 0..SCREEN_WIDTH {
                let x0 = xspace[i];
                let y0 = yspace[j];
                let mut x = 0.0;
                let mut y = 0.0;
                let mut iteration = 0;
                let max_iteration = 1000;
                while x*x + y*y <= 2.0*2.0 && iteration < max_iteration {
                    let xtemp = x*x - y*y + x0;
                    y = 2.0*x*y + y0;
                    x = xtemp;
                    iteration += 1;
                }
                let color = colors[iteration % 10];
                screen[index..=index+3].copy_from_slice(&color);
                index += 4;
            }
        }

}


/// Clear the screen
fn clear(screen: &mut [u8]) {
    for (i, byte) in screen.iter_mut().enumerate() {
        *byte = if i % 4 == 3 { 255 } else { 0 };
    }
}


fn create_window(
    title: &str,
    event_loop: &EventLoop<()>,
) -> (winit::window::Window, u32, u32, f64) {
    // Create a hidden window so we can estimate a good default window size
    let window = winit::window::WindowBuilder::new()
        .with_visible(false)
        .with_title(title)
        .build(event_loop)
        .unwrap();
    let hidpi_factor = window.scale_factor();

    // Get dimensions
    let width = SCREEN_WIDTH as f64;
    let height = SCREEN_HEIGHT as f64;
    let (monitor_width, monitor_height) = {
        if let Some(monitor) = window.current_monitor() {
            let size = monitor.size().to_logical(hidpi_factor);
            (size.width, size.height)
        } else {
            (width, height)
        }
    };
    let scale = (monitor_height / height * 2.0 / 3.0).round().max(1.0);

    // Resize, center, and display the window
    let min_size = PhysicalSize::new(width, height).to_logical::<f64>(hidpi_factor);
    let default_size = LogicalSize::new(width * scale, height * scale);
    let center = LogicalPosition::new(
        (monitor_width - width * scale) / 2.0,
        (monitor_height - height * scale) / 2.0,
    );
    window.set_inner_size(default_size);
    window.set_min_inner_size(Some(min_size));
    window.set_outer_position(center);
    window.set_visible(true);

    let size = default_size.to_physical::<f64>(hidpi_factor);

    (
        window,
        size.width.round() as u32,
        size.height.round() as u32,
        hidpi_factor,
    )
}

fn random_color_space() -> [[u8;4]; 10] {
    let mut colors: [[u8;4]; 10] = [[0x00; 4]; 10];
    colors[0] = [0,0,0,255];
    for i in 1..10 {
        let cr: u8 = rand::thread_rng().gen();
        let cg: u8 = rand::thread_rng().gen();
        let cb: u8 = rand::thread_rng().gen();
        let ca: u8 = rand::thread_rng().gen();

        colors[i] = [cr, cg, cb, ca];
    }

    return colors;
}
