/* License:
* This file is part of term_img.

* term_img is free software:
* you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation,
* either version 3 of the License, or (at your option) any later version.

* term_img is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY;
* without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
* See the GNU General Public License for more details.

*  You should have received a copy of the GNU General Public License along with term_img. If not, see <https://www.gnu.org/licenses/>.
*/

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
