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
