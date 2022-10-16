#[derive(Clone, Copy, Debug)]
pub enum Format {
    Bin,
    Txt,
}

impl std::str::FromStr for Format {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bin" => Ok(Format::Bin),
            "txt" => Ok(Format::Txt),
            _ => { anyhow::bail!("unrecognized Format {:?}; must be \"bin\" or \"txt\"", s); }
        }
    }
}

#[derive(Clone, Debug, clap::Parser)]
pub struct CmdFlags {
    /// Specify which format the input should be read in as.  Values are bin or txt.  Default is bin.
    #[clap(default_value = "bin", env = "SEPT_CAT_IN", long, short='i')]
    pub r#in: Format,
    /// Specify which format the output should be written out as.  Values are bin or txt.  Default is txt.
    #[clap(default_value = "txt", env = "SEPT_CAT_OUT", long, short='o')]
    pub r#out: Format,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    use clap::Parser;
    let cmd_flags = CmdFlags::parse();
    let value = match cmd_flags.r#in {
        Format::Bin => {
            unimplemented!("blah");
        }
        Format::Txt => {
            // NOTE: Reading the whole input into memory is not a scalable solution.
            let input = {
                let mut stdin = std::io::stdin();
                let mut input = String::new();
                use std::io::Read;
                stdin.read_to_string(&mut input)?;
                input
            };
            use std::str::FromStr;
            sept::dy::Value::from_str(&input)?
        }
    };
    log::info!("parsed value:\n{:?}", value);
    use sept::st::Stringifiable;
    log::info!("parsed value stringified:\n{}", value.stringify());
    use sept::dy::Deconstruct;
    let deconstruction = value.deconstructed();
    log::info!("parsed value deconstructed:\n{:?}", deconstruction);
    match cmd_flags.r#out {
        Format::Bin => {
            let mut stdout = std::io::stdout();
            value.serialize(&mut stdout)?;
            Ok(())
        }
        Format::Txt => {
            // NOTE: Writing the entire output into memory is not a scalable solution.
            let output = value.textified();
            use std::io::Write;
            std::io::stdout().write_all(output.as_bytes())?;
            Ok(())
        }
    }
}
