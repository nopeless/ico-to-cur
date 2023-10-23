use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    #[structopt(short, long, help = "Output file")]
    output: Option<PathBuf>,
    #[structopt(short, long, help = "X coordinate")]
    x: Option<u16>,
    #[structopt(short, long, help = "Y coordinate")]
    y: Option<u16>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    let out_path = match args.output {
        Some(path) => path,
        None => PathBuf::from("out.ico"),
    };

    let x = args.x.unwrap_or(0);
    let y = args.y.unwrap_or(0);

    // Eventual output
    let mut curdir = ico::IconDir::new(ico::ResourceType::Cursor);

    // Read input file
    let file = std::fs::File::open(args.input)?;
    let icondir = ico::IconDir::read(file)?;

    let mut icon_image = icondir.entries().first().unwrap().decode()?;

    // Set hostpot
    icon_image.set_cursor_hotspot(Some((x, y)));

    let primary_icon_entry = ico::IconDirEntry::encode(&icon_image)?;

    curdir.add_entry(primary_icon_entry);

    let mut file = std::fs::File::create(out_path)?;

    curdir.write(&mut file)?;

    return Result::Ok(());
}
