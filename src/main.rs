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

mod crush;
mod printing;

use std::io::stdout;

use crate::crush::*;
use crate::printing::*;
use clap::Parser;
use clap::ValueEnum;
use crossterm::execute;
use crossterm::terminal::Clear;

#[derive(Default, Debug, Clone, Copy, ValueEnum)]
enum OutputType {
    #[default]
    TrueColor,
    Ansi,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Arguments {
    pub file: String,
    #[arg(long, short, default_value_t = 4)]
    pub crush_factor: usize,
    #[arg(value_enum, long, short, default_value_t=OutputType::TrueColor)]
    pub output_type: OutputType,
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();
    // println!("{} {}", args.file, args.crush_factor);

    let mut img = image::open(args.file)?.into_rgba32f();

    let img = crush_img(&mut img, args.crush_factor)?.into_rgb8();

    let mut stdout = stdout().lock();
    execute!(stdout, Clear(crossterm::terminal::ClearType::All))?;

    match args.output_type {
        OutputType::TrueColor => print_img_truecolor(&img, &mut stdout),
        OutputType::Ansi => print_img_ansi(&img, &mut stdout),
    }?;

    Ok(())
}
