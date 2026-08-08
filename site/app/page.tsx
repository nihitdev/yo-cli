import { CopyButton } from "./CopyButton";
import { SiteExtras } from "./SiteExtras";

const installs = [
  { label: "Installer", command: "curl --proto '=https' --tlsv1.2 -LsSf https://yo-cli.vercel.app/yo-setup | sh" },
  { label: "Cargo", command: "cargo install yoo" },
  { label: "npm", command: "npm install -g @nihitde_v/yoo" },
  { label: "WinGet", command: "winget install --id Nihitdev.yoo --exact" },
  {
    label: "Scoop",
    command: "scoop bucket add nihitdev https://github.com/nihitdev/scoop-bucket\nscoop install yoo",
  },
];

const commands = [
  ["yoo", "Current project, Git state, and one configured reminder"],
  ["yoo doctor", "Local toolchain and repository checks"],
  ["yoo edit", "Open the current project in your preferred editor"],
  ["yoo project", "Project metadata, source statistics, and Git details"],
  ["yoo fetch", "Development environment and current project"],
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
  ["Python", "pyproject.toml", "pip · uv · Poetry · Pipenv"],
  ["Go", "go.mod", "Go modules"],
  ["Java", "pom.xml · Gradle", "Maven · Gradle"],
  [".NET", ".sln · .csproj", ".NET SDK"],
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
          <h1>Project context,<br /><span>without leaving the terminal.</span></h1>
          <p className="hero-lede">
            A local-first CLI for project metadata, Git status, development environment checks,
            session timers, and configurable reminders.
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
              <span className="version">v1.0.0</span>
            </div>
            <img src="/hero.png" alt="yoo displaying a terminal project session summary" />
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
              <img src={shot.src} alt={`${shot.label} shown in the yoo terminal interface`} loading="lazy" />
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
          <a className="text-link" href="https://github.com/nihitdev/yo-cli#commands">Full command reference <span>→</span></a>
        </div>
      </section>

      <section className="section detection" id="detection">
        <div className="section-heading compact">
          <p className="kicker">Project detection</p>
          <h2>Useful across your whole projects folder.</h2>
          <p>yoo recognizes common repository markers and package managers automatically. Generated folders and dependencies are skipped while source files are counted.</p>
        </div>
        <div className="detection-table" role="table" aria-label="Supported project types">
          <div className="detection-row detection-head" role="row"><span>Project</span><span>Detected from</span><span>Package tooling</span></div>
          {projectTypes.map(([project, marker, tooling]) => (
            <div className="detection-row" role="row" key={project}>
              <strong>{project}</strong><code>{marker}</code><span>{tooling}</span>
            </div>
          ))}
        </div>
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
  "yoo_version": "1.0.0",
  "project": {
    "name": "yoo",
    "language": "Rust",
    "version": "1.0.0"
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
            <a className="text-link" href="https://github.com/nihitdev/yo-cli#configuration">Configuration guide <span>→</span></a>
          </div>
        </div>
      </section>

      <section className="section principles">
        <div className="principle-main">
          <p className="kicker">Project principles</p>
          <h2>Local by design.</h2>
          <p>
            yoo reads local project, environment, and Git information and prints it to your terminal.
            It does not transmit or retain project data.
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
          <a href="https://www.npmjs.com/package/@nihitde_v/yoo">npm</a>
          <a href="https://github.com/nihitdev/yo-cli/releases">Releases</a>
          <a href="https://github.com/nihitdev/yo-cli/blob/main/LICENSE">License</a>
        </div>
      </footer>
    </main>
  );
}
