Name:           yoo
Version:        1.0.0
Release:        0
Summary:        Local CLI for project, Git, and development environment information
License:        GPL-3.0-or-later
URL:            https://github.com/nihitdev/yo-cli
Source0:        https://github.com/nihitdev/yo-cli/archive/refs/tags/v%{version}.tar.gz#/yoo-%{version}.tar.gz
Source1:        vendor.tar.zst
BuildRequires:  cargo-packaging
BuildRequires:  rust
ExclusiveArch:  x86_64

# Keep every cargo macro on the committed Cargo.lock.
%define __cargo_common_opts %{?_smp_mflags} --locked

%description
yoo is a local-first command-line tool for inspecting project metadata, Git
state, and the development environment.

%prep
%autosetup -p1 -a1 -n yo-cli-%{version}
mkdir -p .cargo
cat > .cargo/config.toml <<'EOF'
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
EOF

%build
export CARGO_NET_OFFLINE=true
%cargo_build

%check
export CARGO_NET_OFFLINE=true
%cargo_test

%install
export CARGO_NET_OFFLINE=true
%cargo_install
install -Dm644 LICENSE %{buildroot}%{_licensedir}/%{name}/LICENSE
install -Dm644 README.md %{buildroot}%{_docdir}/%{name}/README.md

%files
%license %{_licensedir}/%{name}/LICENSE
%doc %{_docdir}/%{name}/README.md
%{_bindir}/yoo

%changelog
* Sun Sep 06 2026 Nihit <nihitdev@users.noreply.github.com>
- Initial openSUSE Tumbleweed package.
