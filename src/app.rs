use std::{error::Error, io};

use crate::{
    args::{self, Command, RunOptions},
    config::{self, InitResult},
    content, git,
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
    let path = config::config_path();

    match config::write_default()? {
        InitResult::Created => println!("Created {}", path.display()),
        InitResult::AlreadyExists => println!("Config already exists at {}", path.display()),
    }

    Ok(())
}

fn run(options: RunOptions) -> Result<(), Box<dyn Error>> {
    let config = config::load()?;
    let theme_name = options.theme.as_deref().unwrap_or(&config.theme);
    let theme = Theme::parse(theme_name).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("unknown theme `{theme_name}`; use neon, ocean, or mono"),
        )
    })?;

    let name = options.name.as_deref().unwrap_or(&config.name).trim();
    let typing_speed_ms = if options.fast { 0 } else { config.typing_speed_ms };
    let ui = Ui::new(theme, !options.plain, typing_speed_ms);

    if config.show_art && !options.no_art {
        ui.print_art()?;
        ui.blank_line()?;
    }

    ui.heading("yoo — developer session starter")?;
    ui.type_line(&content::greeting(name))?;
    ui.info("💧", "Hydration:", &content::hydration_reminder())?;
    ui.blank_line()?;

    let directory = std::env::current_dir()?;
    let project = directory
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("current directory");

    ui.divider()?;
    ui.info("📁", "Project:", project)?;

    if config.show_git {
        if let Some(info) = git::inspect(&directory) {
            ui.info("🌿", "Git branch:", &info.branch)?;

            let change_status = if info.changed_files == 0 {
                "clean".to_owned()
            } else {
                format!("{} changed file(s)", info.changed_files)
            };

            ui.info("✏️", "Working tree:", &change_status)?;
        } else {
            ui.info("🌿", "Git:", "not a repository")?;
        }
    }

    ui.divider()?;
    ui.info("💡", "Tip:", &content::tip())?;
    ui.type_line("Go build something fun. 👋")?;

    Ok(())
}
