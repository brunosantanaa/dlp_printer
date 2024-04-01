use std::sync::Arc;
use std::sync::Mutex;

use slint::SharedPixelBuffer;
use slint::Image;
use slint::Rgba8Pixel;
use slint::Weak;

slint::slint!{
    export component App inherits Window {
        in property <image> img <=> canvas.source;
        background: rgb(0,0,0);
        min-width: 800px;
        min-height: 400px;
        
        Rectangle {
            min-width: 200px;
            canvas := Image {}
        }
    }
}

pub struct Screen {
    pub app: App,
    pub weak: Arc<Mutex<Weak<App>>>,
}
pub fn new () -> Screen {
    let app = App::new().unwrap();
    app.window().set_fullscreen(true);
    let weak = Arc::new(Mutex::new(app.as_weak()));

    Screen {app, weak}
}

pub fn run(app: App) {
    app.run().unwrap();
}

pub fn set(weak: Weak<App>, path: String) {
    weak.upgrade_in_event_loop(move |app| {
        let mut img_buf: Option<Image> = None;
        if let Ok(img) = image::open(path.as_str()) {
            let curr_img = img.into_rgba8();
            let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                curr_img.as_raw(),
                curr_img.width(),
                curr_img.height(),
            );
        
            img_buf = Some(Image::from_rgba8(buffer));
        }
        if let Some(image) = img_buf {
            app.set_img(image);
            println!("Image: {}", path);
        }
    }).expect("Failed to upgrade in event loop");
}
