pub struct Config<'a> {
    pub filename: &'a str,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a Vec<String>) -> Result<Config, &'static str> {
        if args.len() > 2 {
            return Err("Too many args");
        }

        if args.len() == 1 {
            return Err("No filename passed");
        }

        Ok(Self {
            filename: args[1].as_str(),
        })
    }
}
