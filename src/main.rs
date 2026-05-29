

fn main()
{
    use brain_needle::*;
    use bnemit::BNEmitterFactoryDefault;
    use bnintermediate::generate_intermediate_representation;
    use bncore::create_bnreader;

    let config = build_config();

    let output = generate_intermediate_representation(create_bnreader(config.file_path)).unwrap_or_else(|err| {
        println!("{err}");
        std::process::exit(1);
    });

    bngen::generate_output::<BNEmitterFactoryDefault>(output, config.context);
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

    #[cfg(debug_assertions)]
    println!("\n{:?}", args);
    
    let config = Config::build(&args);

    #[cfg(debug_assertions)]
    println!("{:?}\n", config);

    return config;
}

#[allow(unused)]
fn mock_config() -> brain_needle::bnconfig::Config
{
    use brain_needle::bnconfig::Config;
    use brain_needle::bnctx::*;

    let input_path = "bf/wrap_pos.bf".into();

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