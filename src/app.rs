use std::{error::Error, io};

use crate::{
    args::{self, Command, FetchOptions, RunOptions, SessionOptions},
    config::{self, WriteResult},
    content, doctor, fetch, git, timer, tips,
    ui::{Theme, Ui},
};

pub fn execute(command: Command) -> Result<(), Box<dyn Error>> {
    match command {
        Command::Run(options) => run(options),
        Command::Init => init(),
        Command::ConfigPath => {
            println!("{}", config::config_path().display());
            Ok(())
        }
        Command::Doctor => {
            let directory = std::env::current_dir()?;
            doctor::print(&doctor::collect(&directory));
            Ok(())
        }
        Command::Fetch(options) => run_fetch(options),
        Command::Session(options) => session(options),
        Command::Tip(pack) => print_tip(pack),
        Command::Tips => list_tips(),
        Command::Help => {
            print!("{}", args::help_text());
            Ok(())
        }
        Command::Version => {
            println!("yoo {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
    }
}

fn init() -> Result<(), Box<dyn Error>> {
    let config_path = config::config_path();
    let tip_path = config::tip_packs_dir().join("community.yaml");

    match config::write_default()? {
        WriteResult::Created => println!("Created {}", config_path.display()),
        WriteResult::AlreadyExists => {
            println!("Config already exists at {}", config_path.display())
        }
    }

    match tips::write_sample_pack()? {
        WriteResult::Created => println!("Created sample tip pack {}", tip_path.display()),
        WriteResult::AlreadyExists => {
            println!("Sample tip pack already exists at {}", tip_path.display())
        }
    }

    Ok(())
}

fn run(options: RunOptions) -> Result<(), Box<dyn Error>> {
    let config = config::load()?;
    let ui = ui_for_run(&config, &options)?;

    let name = options
        .name
        .as_deref()
        .unwrap_or(&config.profile.name)
        .trim();

    if config.appearance.ascii && !options.no_art {
        ui.print_art()?;
        ui.blank_line()?;
    }

    ui.heading("yoo — developer session starter")?;
    ui.type_line(&content::greeting(name))?;

    if config.hydration.enabled {
        ui.info("💧", "Hydration:", &content::hydration_reminder())?;
    }

    ui.blank_line()?;

    let directory = std::env::current_dir()?;
    let project = directory
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("current directory");

    ui.divider()?;
    ui.info("📁", "Project:", project)?;

    if config.git.show_branch {
        if let Some(info) = git::inspect(&directory) {
            ui.info("🌿", "Git branch:", &info.branch)?;

            if config.git.show_status {
                let change_status = if info.changed_files == 0 {
                    "clean".to_owned()
                } else {
                    format!("{} changed file(s)", info.changed_files)
                };
                ui.info("✏️", "Working tree:", &change_status)?;
            }
        } else {
            ui.info("🌿", "Git:", "not a repository")?;
        }
    }

    if config.tips.enabled {
        ui.divider()?;
        ui.info("💡", "Tip:", &tips::random_tip(&config.tips.pack)?)?;
    }

    ui.type_line("Go build something fun. 👋")?;
    Ok(())
}

fn run_fetch(options: FetchOptions) -> Result<(), Box<dyn Error>> {
    let directory = std::env::current_dir()?;
    let report = fetch::collect(&directory);

    if options.json {
        println!("{}", fetch::to_json(&report)?);
        return Ok(());
    }

    let config = config::load()?;
    let ui = ui_for_fetch(&config, &options)?;

    if config.appearance.ascii && !options.no_art {
        ui.print_art()?;
        ui.blank_line()?;
    }

    fetch::print(&report, &ui)?;
    Ok(())
}

fn session(options: SessionOptions) -> Result<(), Box<dyn Error>> {
    let config = config::load()?;
    let minutes = options.minutes.unwrap_or(config.session.default_minutes);
    timer::validate_minutes(minutes)
        .map_err(|message| io::Error::new(io::ErrorKind::InvalidInput, message))?;

    timer::start(minutes, config.session.show_complete_message)?;
    Ok(())
}

fn print_tip(pack: Option<String>) -> Result<(), Box<dyn Error>> {
    let config = config::load()?;
    let pack = pack.unwrap_or(config.tips.pack);
    println!("💡 {}", tips::random_tip(&pack)?);
    Ok(())
}

fn list_tips() -> Result<(), Box<dyn Error>> {
    println!("Available yoo tip packs:\n");

    for pack in tips::list_packs()? {
        let source = match pack.source {
            tips::PackSource::BuiltIn => "built-in",
            tips::PackSource::Local => "local",
        };
        println!("- {:<12} [{}] {}", pack.name, source, pack.description);
    }

    println!("\nAdd community packs as YAML files in:");
    println!("{}", config::tip_packs_dir().display());
    Ok(())
}

fn ui_for_run(config: &config::Config, options: &RunOptions) -> Result<Ui, io::Error> {
    ui_for_display(
        config,
        options.theme.as_deref(),
        options.plain,
        if options.fast {
            0
        } else {
            config.appearance.typing_speed_ms
        },
    )
}

fn ui_for_fetch(config: &config::Config, options: &FetchOptions) -> Result<Ui, io::Error> {
    ui_for_display(config, options.theme.as_deref(), options.plain, 0)
}

fn ui_for_display(
    config: &config::Config,
    theme_override: Option<&str>,
    plain: bool,
    typing_speed_ms: u64,
) -> Result<Ui, io::Error> {
    let theme_name = theme_override.unwrap_or(&config.appearance.theme);
    let theme = Theme::parse(theme_name).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "unknown theme `{theme_name}`; use one of: {}",
                Theme::names().join(", ")
            ),
        )
    })?;

    Ok(Ui::new(
        theme,
        config.appearance.colors && !plain,
        typing_speed_ms,
    ))
}
