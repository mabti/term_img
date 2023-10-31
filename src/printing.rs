use crossterm::{
    cursor::MoveTo,
    style::{self, Color as TermColor, Stylize},
    QueueableCommand,
};
use image::{Rgb, RgbImage};
use std::io::{Result as IOResult, StdoutLock, Write};

fn print_pixel(stdout: &mut StdoutLock, x: u16, y: u16, color: TermColor) -> IOResult<()> {
    stdout
        .queue(MoveTo(x, y))?
        .queue(style::PrintStyledContent(" ".on(color)))?;

    Ok(())
}

pub fn print_img_truecolor(img: &RgbImage, stdout: &mut StdoutLock) -> IOResult<()> {
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let &Rgb([r, g, b]) = img.get_pixel(x, y);

            let color = TermColor::Rgb { r, g, b };
            print_pixel(stdout, x as u16, y as u16, color)?;
        }
    }

    stdout.flush()?;
    Ok(())
}

const RGB_TERM_COLOURS: [Rgb<u8>; 16] = [
    Rgb([0, 0, 0]),
    Rgb([128, 0, 0]),
    Rgb([0, 128, 0]),
    Rgb([128, 128, 0]),
    Rgb([0, 0, 128]),
    Rgb([128, 0, 128]),
    Rgb([0, 128, 128]),
    Rgb([192, 192, 192]),
    Rgb([128, 128, 128]),
    Rgb([255, 0, 0]),
    Rgb([0, 255, 0]),
    Rgb([255, 255, 0]),
    Rgb([0, 0, 255]),
    Rgb([255, 0, 255]),
    Rgb([0, 255, 255]),
    Rgb([255, 255, 255]),
];

const TERM_COLOURS: [TermColor; 16] = [
    TermColor::Black,
    TermColor::DarkRed,
    TermColor::DarkGreen,
    TermColor::DarkYellow,
    TermColor::DarkBlue,
    TermColor::DarkMagenta,
    TermColor::DarkCyan,
    TermColor::Grey,
    TermColor::DarkGrey,
    TermColor::Red,
    TermColor::Green,
    TermColor::Yellow,
    TermColor::Blue,
    TermColor::Magenta,
    TermColor::Cyan,
    TermColor::White,
];

fn colour_distance(a: Rgb<u8>, b: Rgb<u8>) -> isize {
    let mut dist_raw: isize = 0;

    for i in 0..3 {
        let comp_dist_sqrt = a.0[i] as isize - b.0[i] as isize;
        let comp_dist = comp_dist_sqrt * comp_dist_sqrt;

        dist_raw += comp_dist;
    }

    dist_raw
}

fn calc_closest_term_color(rgb: Rgb<u8>) -> usize {
    let mut closest_idx = 0;
    let mut closest_dist = isize::MAX;

    for (i, color) in RGB_TERM_COLOURS.into_iter().enumerate() {
        let dist = colour_distance(rgb, color);

        if dist < closest_dist {
            closest_idx = i;
            closest_dist = dist;
        }
    }

    closest_idx
}

pub fn print_img_ansi(img: &RgbImage, stdout: &mut StdoutLock) -> IOResult<()> {
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let closest_colour_idx = calc_closest_term_color(*img.get_pixel(x, y));
            let colour = TERM_COLOURS[closest_colour_idx];

            print_pixel(stdout, x as u16, y as u16, colour)?;
        }
    }

    Ok(())
}
