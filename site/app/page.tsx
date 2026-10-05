import { CopyButton } from "./CopyButton";
import { SiteExtras } from "./SiteExtras";
import Image from "next/image";

const installs = [
  { label: "Arch / AUR", command: "yay -S yoo-bin\n# or\nparu -S yoo-bin" },
  { label: "Installer", command: "curl --proto '=https' --tlsv1.2 -LsSf https://yo-cli.vercel.app/yo-setup | sh" },
  { label: "APT", command: "curl -fsSL https://yo-cli.vercel.app/apt/setup.sh | sudo sh\nsudo apt install yoo" },
  { label: "DNF", command: "sudo dnf config-manager addrepo --from-repofile=https://yo-cli.vercel.app/rpm/yoo.repo\nsudo dnf install yoo" },
  { label: "Alpine", command: "echo 'https://yo-cli.vercel.app/alpine' | sudo tee -a /etc/apk/repositories\nsudo apk add yoo" },
  { label: "openSUSE", command: "sudo zypper ar -f https://yo-cli.vercel.app/opensuse yoo\nsudo zypper refresh\nsudo zypper install yoo" },
  { label: "Flatpak", command: "flatpak remote-add --user --if-not-exists yoo https://yo-cli.vercel.app/flatpak/yoo.flatpakrepo\nflatpak install --user yoo io.github.nihitdev.yoo" },
  { label: "Nix", command: "nix run github:nihitdev/yo-cli" },
  { label: "Cargo", command: "cargo install yoo" },
  { label: "npm", command: "npm install -g @nihit_dev/yoo" },
  { label: "WinGet", command: "winget install --id Nihitdev.yoo --exact" },
];

const packageStatuses = [
  ["Cargo", "Official registry", "Linux · macOS · Windows"],
  ["npm / pnpm / Bun", "Official npm package", "Cross-platform wrapper"],
  ["APT · DNF · Alpine · openSUSE", "Self-hosted repositories", "Linux x86_64"],
  ["Homebrew · AUR", "Community packaging", "macOS · Arch Linux"],
  ["Nix · Flatpak", "GitHub flake · self-hosted repo", "Source build · static repository"],
  ["WinGet · Chocolatey", "Community channels", "Windows"],
  ["Void / XBPS", "GitHub Release artifact", "Build and validation automated"],
  ["Snap", "GitHub Release artifact", "Store publishing disabled"],
];

const commands = [
  ["yoo", "Current project, Git state, and one configured reminder"],
  ["yoo doctor", "Local toolchain and repository checks"],
  ["yoo edit", "Open the current project in your preferred editor"],
  ["yoo project", "Project metadata, source statistics, and Git details"],
  ["yoo fetch", "Development environment and current project"],
  ["yoo snapshot", "Save project, Git, source-count, and tool-version information locally"],
  ["yoo snapshot list", "List saved snapshots with their IDs"],
  ["yoo snapshot compare", "Compare the two newest snapshots"],
  ["yoo snapshot compare --current", "Compare the latest snapshot with the current project"],
  ["yoo session 25", "Local coding-session timer"],
  ["yoo completions bash", "Generate completions for your shell"],
];

const features = [
  {
    number: "01",
    title: "Know the repository",
    description: "See the detected language, version, package manager, source-file counts, branch, and working-tree state in one scan.",
    command: "yoo project",
  },
  {
    number: "02",
    title: "Check the toolchain",
    description: "Confirm that Git, language tools, formatters, linters, project detection, and your yoo configuration are ready to use.",
    command: "yoo doctor",
  },
  {
    number: "03",
    title: "Start with context",
    description: "Open a coding session with the project name, branch, pending changes, and one useful reminder already in view.",
    command: "yoo --fast",
  },
  {
    number: "04",
    title: "Automate the output",
    description: "Use stable, undecorated JSON in scripts, editor integrations, status bars, and your own developer tooling.",
    command: "yoo project --json",
  },
];

const projectTypes = [
  ["Rust", "Cargo.toml", "Cargo"],
  ["Node.js", "package.json", "npm · pnpm · Yarn · Bun"],
  ["Python", "pyproject.toml · requirements.txt · Pipfile · setup.py · setup.cfg", "pip · uv · Poetry · Pipenv"],
  ["Go", "go.mod", "Go modules"],
  ["Java", "pom.xml · Gradle", "Maven · Gradle"],
  [".NET", ".sln · .csproj", ".NET SDK"],
  ["Zig", "build.zig · build.zig.zon", "Zig"],
  ["Ruby", "Gemfile", "Bundler"],
  ["PHP", "composer.json", "Composer"],
  ["Swift", "Package.swift", "Swift Package Manager"],
  ["Dart", "pubspec.yaml", "pub"],
  ["Elixir", "mix.exs", "Mix"],
  ["C/C++", "CMakeLists.txt", "CMake"],
];

const screenshots = [
  { src: "/hero.png", label: "Session summary", command: "yoo --fast" },
  { src: "/doctor.png", label: "Toolchain checks", command: "yoo doctor" },
  { src: "/projects.png", label: "Project overview", command: "yoo project" },
  { src: "/fetch.png", label: "Environment report", command: "yoo fetch" },
  { src: "/session.png", label: "Session timer", command: "yoo session 25" },
  { src: "/tips.png", label: "Tip packs", command: "yoo tips" },
  { src: "/edit.png", label: "Editor launch", command: "yoo edit" },
];

export default function Home() {
  return (
    <main>
      <SiteExtras />
      <header className="nav-shell">
        <nav className="nav" aria-label="Main navigation">
          <a className="brand" href="#top" aria-label="yoo home">
            <span className="brand-mark">yoo</span><span className="cursor">_</span>
          </a>
          <div className="nav-links">
            <a href="#features">Features</a>
            <a href="#install">Install</a>
            <a href="#packages">Packages</a>
            <a href="#commands">Commands</a>
            <a href="#configure">Configure</a>
            <a href="https://github.com/nihitdev/yo-cli/tree/main/docs">Docs</a>
          </div>
          <a className="github-link" href="https://github.com/nihitdev/yo-cli">
            GitHub <span aria-hidden="true">↗</span>
          </a>
        </nav>
      </header>

      <section className="hero" id="top">
        <div className="hero-copy">
          <div className="eyebrow"><span className="status-dot" /> Open source · GPL-3.0-or-later</div>
          <h1>What the hell is going on<br /><span>with this project?</span></h1>
          <p className="hero-lede">
            yoo is a fast, local-first CLI for project metadata, Git state, development environment
            checks, local snapshots, session timers, and configurable reminders.
          </p>
          <div className="hero-actions">
            <a className="button primary" href="#install">Install yoo</a>
            <a className="button secondary" href="https://github.com/nihitdev/yo-cli/releases/latest">Latest release <span>↗</span></a>
          </div>
          <div className="project-facts" aria-label="Project facts">
            <span>Rust</span><span>Windows</span><span>Linux</span><span>macOS</span><span>No telemetry</span>
          </div>
        </div>
        <div className="hero-visual">
          <div className="terminal-frame">
            <div className="terminal-bar">
              <div className="traffic"><i /><i /><i /></div>
              <span>~/projects/yoo</span>
              <span className="version">Session preview</span>
            </div>
            <Image src="/hero.png" alt="yoo displaying a terminal project session summary" width={1366} height={768} priority />
          </div>
          <div className="accent-grid" aria-hidden="true" />
        </div>
      </section>

      <section className="section feature-section" id="features">
        <div className="section-heading">
          <p className="kicker">What yoo does</p>
          <h2>Your project&apos;s vital signs, at a glance.</h2>
          <p>Use one focused tool instead of stitching together a handful of commands whenever you enter a repository.</p>
        </div>
        <div className="feature-grid">
          {features.map((feature) => (
            <article className="feature-card" key={feature.number}>
              <span className="feature-number">{feature.number}</span>
              <h3>{feature.title}</h3>
              <p>{feature.description}</p>
              <code>$ {feature.command}</code>
            </article>
          ))}
        </div>
      </section>

      <section className="section screenshots" id="screenshots">
        <div className="section-heading">
          <p className="kicker">Terminal output</p>
          <h2>One command for each view.</h2>
          <p>Every report stays local. JSON output is available for scripts and editor integrations.</p>
        </div>
        <div className="screenshot-grid">
          {screenshots.map((shot) => (
            <article className="screenshot-card" key={shot.src}>
              <div className="screenshot-meta">
                <span>{shot.label}</span>
                <code>$ {shot.command}</code>
              </div>
              <Image src={shot.src} alt={`${shot.label} shown in the yoo terminal interface`} width={1366} height={768} />
            </article>
          ))}
        </div>
      </section>

      <section className="section install" id="install">
        <div className="section-heading compact">
          <p className="kicker">Installation</p>
          <h2>Use the package manager already on your system.</h2>
        </div>
        <div className="install-layout">
          <div className="install-list">
            {installs.map((item, index) => (
              <div className={`install-row ${index === 0 ? "featured" : ""}`} key={item.label}>
                <span className="install-label">{item.label}</span>
                <code>{item.command}</code>
                <CopyButton value={item.command} label={item.label} />
              </div>
            ))}
          </div>
          <aside className="verified-card">
            <span className="check">✓</span>
            <h3>Verified downloads</h3>
            <p>The installer checks release binaries against the published SHA-256 checksum before installation.</p>
            <a href="https://github.com/nihitdev/yo-cli/blob/main/docs/installation.md">Installation guide <span>→</span></a>
          </aside>
        </div>
      </section>

      <section className="section packages" id="packages">
        <div className="section-heading compact">
          <p className="kicker">Distribution</p>
          <h2>Pick the channel that fits your machine.</h2>
          <p>Package status is stated plainly: self-hosted repositories and release artifacts are maintained here, while community channels may have their own review timelines.</p>
        </div>
        <div className="package-status-grid">
          {packageStatuses.map(([name, status, detail]) => (
            <article className="package-status-card" key={name}>
              <strong>{name}</strong>
              <span>{status}</span>
              <small>{detail}</small>
            </article>
          ))}
        </div>
      </section>

      <section className="section commands" id="commands">
        <div className="section-heading compact">
          <p className="kicker">Command reference</p>
          <h2>Small command surface. Plain output.</h2>
        </div>
        <div className="command-table">
          {commands.map(([command, description]) => (
            <div className="command-row" key={command}>
              <code>{command}</code>
              <span>{description}</span>
              <span className="arrow" aria-hidden="true">→</span>
            </div>
          ))}
        </div>
        <div className="reference-note">
          <div>
            <strong>More built in</strong>
            <p><code>yoo status</code>, <code>yoo tip</code>, <code>yoo tips</code>, <code>yoo init</code>, <code>yoo config</code>, and <code>yoo help</code>.</p>
          </div>
          <a className="text-link" href="https://github.com/nihitdev/yo-cli/blob/main/docs/commands.md">Full command reference <span>→</span></a>
        </div>
      </section>

      <section className="section detection" id="detection">
        <div className="section-heading compact">
          <p className="kicker">Project detection</p>
          <h2>Useful across your whole projects folder.</h2>
          <p>From Rust and Zig to Ruby, Swift, Dart, and Elixir, yoo detects project markers and selects the matching toolchain checks. Source counts skip common generated folders and dependencies.</p>
        </div>
        <div className="detection-table" role="table" aria-label="Supported project types">
          <div className="detection-row detection-head" role="row"><span role="columnheader">Project</span><span role="columnheader">Detected from</span><span role="columnheader">Package / build tooling</span></div>
          {projectTypes.map(([project, marker, tooling]) => (
            <div className="detection-row" role="row" key={project}>
              <strong role="cell">{project}</strong><code role="cell">{marker}</code><span role="cell">{tooling}</span>
            </div>
          ))}
        </div>
        <p className="detection-note">Run yoo from the project directory. In mixed projects, the first matching manifest wins. CMake is treated as a C/C++ project; Dart detection includes Flutter projects. <a className="text-link" href="https://github.com/nihitdev/yo-cli/blob/main/docs/project-detection.md">Detection details →</a></p>
      </section>

      <section className="section snapshots" id="snapshots">
        <div className="section-heading compact">
          <p className="kicker">Local snapshots</p>
          <h2>See what changed since you started.</h2>
          <p>Save a point in time, then compare Git state, source counts, project version, and tool versions as you work.</p>
        </div>
        <div className="snapshot-steps">
          <article className="feature-card">
            <span className="feature-number">01 · Save</span>
            <h3>Capture the project.</h3>
            <p>Record a local snapshot before a refactor or at the start of a session.</p>
            <code>yoo snapshot</code>
          </article>
          <article className="feature-card">
            <span className="feature-number">02 · Browse</span>
            <h3>Find a saved moment.</h3>
            <p>List snapshots newest first. Each has an ID you can use to select a comparison.</p>
            <code>yoo snapshot list</code>
          </article>
          <article className="feature-card">
            <span className="feature-number">03 · Compare</span>
            <h3>Check your progress.</h3>
            <p>Compare the latest snapshot with the current project without saving another file.</p>
            <code>yoo snapshot compare --current</code>
          </article>
        </div>
        <p className="detection-note">Snapshots stay in <code>.yoo/snapshots/</code>. The first save creates an ignore file to keep new snapshots out of Git; existing ignore rules are preserved. Snapshots record project information, not backups of your source files.</p>
      </section>

      <section className="section automation" id="automation">
        <div className="automation-copy">
          <p className="kicker">Made for automation</p>
          <h2>Human-friendly in a terminal. Predictable in a script.</h2>
          <p>Switch project and environment reports to JSON when another tool needs the data. Decorative display flags stay separate, so machine-readable output remains clean.</p>
          <div className="automation-actions">
            <code>yoo fetch --json</code>
            <code>yoo project --json</code>
          </div>
        </div>
        <pre className="json-card" aria-label="Example yoo JSON output"><code>{`{
  "yoo_version": "1.1.2",
  "project": {
    "name": "yoo",
    "language": "Rust",
    "version": "1.1.2"
  },
  "git": {
    "branch": "main",
    "changed_files": 0
  }
}`}</code></pre>
      </section>

      <section className="section configure" id="configure">
        <div className="section-heading compact">
          <p className="kicker">Make it yours</p>
          <h2>Configure once. Stay local.</h2>
          <p>Run <code>yoo init</code> to create a readable TOML config and a sample tip pack. Choose a theme, tune the session timer, hide ASCII art, or add reminders for your own team.</p>
        </div>
        <div className="configure-grid">
          <div className="config-card">
            <div className="config-bar"><span>config.toml</span><span>~/.config/yoo</span></div>
            <pre><code>{`[appearance]
theme = "catppuccin"
ascii = true
colors = true

[editor]
command = "code"

[git]
show_branch = true
show_status = true

[session]
default_minutes = 25`}</code></pre>
          </div>
          <div className="theme-card">
            <span className="feature-number">09 themes included</span>
            <h3>Match your terminal.</h3>
            <p>Neon, Ocean, Mono, Dracula, Tokyo Night, Gruvbox, Nord, Rosé Pine, and Catppuccin ship with yoo.</p>
            <div className="theme-swatches" aria-label="Theme color samples">
              <i /><i /><i /><i /><i /><i /><i /><i /><i />
            </div>
            <a className="text-link" href="https://github.com/nihitdev/yo-cli/blob/main/docs/configuration.md">Configuration guide <span>→</span></a>
          </div>
        </div>
      </section>

      <section className="section principles">
        <div className="principle-main">
          <p className="kicker">Project principles</p>
          <h2>Local by design.</h2>
          <p>
            yoo reads local project, environment, and Git information and prints it to your terminal.
            It does not upload project data. Snapshots are saved locally only when you ask.
          </p>
          <a className="text-link" href="https://github.com/nihitdev/yo-cli/blob/main/CONTRIBUTING.md">Contributing guide <span>→</span></a>
        </div>
        <div className="principle-grid">
          <div><strong>00</strong><span>Network calls during normal use</span></div>
          <div><strong>00</strong><span>Accounts or required services</span></div>
          <div><strong>01</strong><span>Small Rust executable</span></div>
          <div><strong>09</strong><span>Built-in color themes</span></div>
        </div>
      </section>

      <section className="section final-cta">
        <p className="kicker">Ready when you are</p>
        <h2>Meet the project you&apos;re in.</h2>
        <p>Install yoo, open a repository, and run your first local project summary.</p>
        <div className="hero-actions">
          <a className="button primary" href="#install">Choose an install method</a>
          <a className="button secondary" href="https://github.com/nihitdev/yo-cli/blob/main/docs/README.md">Read the docs <span>↗</span></a>
        </div>
      </section>

      <footer>
        <a className="brand footer-brand" href="#top"><span className="brand-mark">yoo</span><span className="cursor">_</span></a>
        <p>Local project and development environment information.</p>
        <div className="footer-links">
          <a href="https://crates.io/crates/yoo">crates.io</a>
          <a href="https://www.npmjs.com/package/@nihit_dev/yoo">npm</a>
          <a href="https://github.com/nihitdev/yo-cli/releases">Releases</a>
          <a href="https://github.com/nihitdev/yo-cli/blob/main/LICENSE">License</a>
        </div>
      </footer>
    </main>
  );
}
