import { CopyButton } from "./CopyButton";

const installs = [
  { label: "Installer", command: "curl -LsSf https://raw.githubusercontent.com/nihitdev/yo-cli/main/install.sh | sh" },
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
  ["yoo project", "Project metadata, source statistics, and Git details"],
  ["yoo fetch", "Development environment and current project"],
  ["yoo session 25", "Local coding-session timer"],
  ["yoo completions bash", "Generate completions for your shell"],
];

const screenshots = [
  { src: "/hero.png", label: "Session summary", command: "yoo --fast" },
  { src: "/doctor.png", label: "Toolchain checks", command: "yoo doctor" },
  { src: "/projects.png", label: "Project overview", command: "yoo project" },
  { src: "/fetch.png", label: "Environment report", command: "yoo fetch" },
  { src: "/session.png", label: "Session timer", command: "yoo session 25" },
  { src: "/tips.png", label: "Tip packs", command: "yoo tips" },
];

export default function Home() {
  return (
    <main>
      <header className="nav-shell">
        <nav className="nav" aria-label="Main navigation">
          <a className="brand" href="#top" aria-label="yoo home">
            <span className="brand-mark">yoo</span><span className="cursor">_</span>
          </a>
          <div className="nav-links">
            <a href="#screenshots">Screenshots</a>
            <a href="#install">Install</a>
            <a href="#commands">Commands</a>
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
              <span className="version">v0.9.0</span>
            </div>
            <img src="/hero.png" alt="yoo displaying a terminal project session summary" />
          </div>
          <div className="accent-grid" aria-hidden="true" />
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
