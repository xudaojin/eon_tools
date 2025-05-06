use clap::Parser;

#[derive(Parser)]
#[command(name = "eontop", author = "daojin.xu101@gmail.com", about = "eontop tool")]
pub struct Eontop {
    #[arg(long, short, default_value_t = true, help = " show all top" )]
    all: bool,

    #[arg(long, short, default_value_t = false, help = " only gpu top" )]
    gpu: bool,

    #[arg(long, short, default_value_t = false, help = " only io top" )]
    io: bool,

    #[arg(long, short, default_value_t = false, help = " only version top" )]
    version: bool,

    #[arg(long, short, default_value_t = false, help = " only network top" )]
    network: bool,
    
    #[arg(long, short, default_value_t = false, help = " only cpu top" )]
    cpu: bool
}


impl Eontop {
    pub fn new() -> Self{
        Eontop::parse()
    }
    
    pub fn run(&self) {
        if self.all {
            
        }
    }
}