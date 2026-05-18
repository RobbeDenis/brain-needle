
fn main()
{
    use brain_needle::*;
    use bnctx::*;
    use bncore::Config;
    use bnemit::BNEmitterFactoryDefault;

    #[cfg(not(feature = "args"))]
    let config: Config = Config { file_path: "src-bf\\test\\wrap_pos.bf".into() };

    #[cfg(feature = "args")]
    let config = {
        let args: Vec<String> = std::env::args().collect();
        Config::build(&args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {err}");
            std::process::exit(1);
        })
    };

    let tokens = bnlex::tokenize_file_from_path(config.file_path).unwrap_or_else(|err| {
        println!("Lexer error: {err}");
        std::process::exit(1);
    });

    let flat_instr_tree = bnparse::create_flat_instr_tree_from_tokens(tokens).unwrap_or_else(|err| {
        println!("Parsing error: {err}");
        std::process::exit(1);
    });

    let target_ctx = TargetContext { 
        os:     TargetOS::Linux, 
        arch:   Architecture::X86_64, 
        // format: OutputFormat::Assembly(AsmFlavor::NASM),
        // dest:   OutputDest::File(Some("output\\out.asm".into()))
        // dest:   OutputDest::File(Some("bnout.asm".into()))
        format: OutputFormat::Interpreted,
        dest:   OutputDest::Stdout
        // dest:   OutputDest::File(Some("output\\out.txt".into()))
    };

    bngen::generate_output::<BNEmitterFactoryDefault>(flat_instr_tree, target_ctx);
}