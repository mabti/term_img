use image::{DynamicImage, Rgba, Rgba32FImage};

fn crush_pixel(img_buf: &mut Rgba32FImage, crush_factor: usize, x: u32, y: u32) -> Rgba<f32> {
    let crush_factor = crush_factor as u32;

    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;
    let mut a = 0.0;

    let crush_sqr = (crush_factor * crush_factor) as f32;

    for y_o in 0..crush_factor {
        for x_o in 0..crush_factor {
            let Rgba([cr, cg, cb, ca]) = img_buf.get_pixel(x + x_o, y + y_o);

            r += cr;
            g += cg;
            b += cb;
            a += ca;
        }
    }

    r /= crush_sqr;
    g /= crush_sqr;
    b /= crush_sqr;
    a /= crush_sqr;

    Rgba([r, g, b, a])
}

pub fn crush_img(img_buf: &mut Rgba32FImage, crush_factor: usize) -> anyhow::Result<DynamicImage> {
    let crush_u32 = crush_factor as u32;

    let width = img_buf.width();
    let height = img_buf.height();

    let max_x = width - crush_u32;
    let max_y = height - crush_u32;

    let mut buf = Rgba32FImage::new(width / crush_u32, height / crush_u32);

    for y in (0..max_y).step_by(crush_factor) {
        for x in (0..max_x).step_by(crush_factor) {
            buf.put_pixel(
                x / crush_u32,
                y / crush_u32,
                crush_pixel(img_buf, crush_factor, x, y),
            );
        }
    }

    Ok(DynamicImage::ImageRgba32F(buf))
}
