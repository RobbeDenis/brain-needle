

fn main()
{
    use brain_needle::*;
    use bnemit::BNEmitterFactoryDefault;

    let config = build_config();

    let tokens = bnlex::tokenize_file_from_path(config.file_path).unwrap_or_else(|err| {
        println!("Lexer error: {err}");
        std::process::exit(1);
    });

    let flat_instr_tree = bnparse::create_flat_instr_tree_from_tokens(tokens).unwrap_or_else(|err| {
        println!("Parsing error: {err}");
        std::process::exit(1);
    });

    bngen::generate_output::<BNEmitterFactoryDefault>(flat_instr_tree, config.context);
}

fn build_config() -> brain_needle::bnconfig::Config
{
    #[cfg(feature = "mock-config")]
    return mock_config();
    
    #[cfg(not(feature = "mock-config"))]
    return parsed_config();
}

#[allow(unused)]
fn parsed_config() -> brain_needle::bnconfig::Config
{
    use brain_needle::bnconfig::*;

    let args = Args::parse_from_env();
    println!("{:?}", args);
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Config error: {err}");
        std::process::exit(1);
    });
    println!("{:?}", config);

    return config;
}

#[allow(unused)]
fn mock_config() -> brain_needle::bnconfig::Config
{
    use brain_needle::bnconfig::Config;
    use brain_needle::bnctx::*;

    let input_path = "src-bf\\test\\wrap_pos.bf".into();

    let ctx = TargetContext { 
        os:     TargetOS::Linux, 
        arch:   Architecture::X86_64, 
        // format: OutputFormat::Assembly(AsmFlavor::NASM),
        format: OutputFormat::Interpreted,
        dest:   OutputDest::Stdout
        // dest:   OutputDest::File(Some("output\\out.txt".into()))
    };

    return Config { file_path: input_path, context: ctx };
}