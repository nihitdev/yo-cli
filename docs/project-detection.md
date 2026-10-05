# Project detection

[← Back to yoo](../README.md)

Detection reads markers in the current directory without running project build
scripts. In mixed projects, the first matching marker below wins; yoo does not
aggregate multiple languages or search parent directories for a manifest.

| Project type | Markers (in precedence order) | Package/build tool |
| --- | --- | --- |
| Rust | `Cargo.toml` | Cargo |
| Node.js | `package.json` | npm, pnpm, Yarn, Bun |
| Python | `pyproject.toml` | pip, uv, Poetry, Pipenv |
| Go | `go.mod` | Go modules |
| Java | `pom.xml`, `build.gradle.kts`, `build.gradle` | Maven, Gradle |
| .NET | `*.sln`, `*.csproj` (case-insensitive, first alphabetically) | .NET SDK |
| Zig | `build.zig`, `build.zig.zon` | Zig |
| Ruby | `Gemfile` | Bundler |
| PHP | `composer.json` | Composer |
| Swift | `Package.swift` | Swift Package Manager |
| Dart | `pubspec.yaml` | pub |
| Elixir | `mix.exs` | Mix |
| C/C++ | `CMakeLists.txt` | CMake |
| Python (fallback) | `requirements.txt`, `Pipfile`, `setup.py`, `setup.cfg` | pip, uv, Poetry, Pipenv |

CMake is treated as a C/C++ project heuristic. Dart detection also recognizes
Flutter projects through `pubspec.yaml`; the health check checks Dart.
Rust and Node.js reports extract package names and versions; other types use
the directory name and report the manifest without evaluating it. Cargo
workspace-inherited metadata is not resolved.

Source counts include Zig, Ruby, PHP, Swift, Dart and Elixir extensions and skip
`.zig-cache`, `zig-cache`, `zig-out`, `.build`, `.dart_tool`, `_build` and `deps`,
in addition to the generated/vendor directories listed in the
[command reference](commands.md).
