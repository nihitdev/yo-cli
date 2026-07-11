use roxmltree::{Document, Node};
use toml::Value;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Metadata {
    pub name: Option<String>,
    pub version: Option<String>,
    pub edition: Option<String>,
    pub license: Option<String>,
}

pub fn cargo(contents: &str) -> Metadata {
    let Ok(document) = contents.parse::<Value>() else {
        return Metadata::default();
    };

    Metadata {
        name: cargo_value(&document, "name"),
        version: cargo_value(&document, "version"),
        edition: cargo_value(&document, "edition"),
        license: cargo_value(&document, "license"),
    }
}

pub fn pyproject(contents: &str) -> Metadata {
    let Ok(document) = contents.parse::<Value>() else {
        return Metadata::default();
    };

    let project = document.get("project");
    let poetry = document.get("tool").and_then(|tool| tool.get("poetry"));

    Metadata {
        name: string_at(project, "name").or_else(|| string_at(poetry, "name")),
        version: string_at(project, "version").or_else(|| string_at(poetry, "version")),
        license: license_at(project).or_else(|| string_at(poetry, "license")),
        edition: None,
    }
}

pub fn package_json(contents: &str) -> Metadata {
    let Ok(document) = serde_json::from_str::<serde_json::Value>(contents) else {
        return Metadata::default();
    };

    Metadata {
        name: json_string(&document, "name"),
        version: json_string(&document, "version"),
        license: json_string(&document, "license"),
        edition: None,
    }
}

pub fn go_mod(contents: &str) -> Metadata {
    let module = contents.lines().find_map(|line| {
        let mut parts = line.split_whitespace();
        if parts.next()? != "module" {
            return None;
        }
        let value = parts.next()?;

        (!value.is_empty()).then(|| value.trim_matches('"').to_owned())
    });

    Metadata {
        name: module.as_deref().and_then(module_name),
        ..Metadata::default()
    }
}

pub fn maven(contents: &str) -> Metadata {
    let Ok(document) = Document::parse(contents) else {
        return Metadata::default();
    };
    let root = document.root_element();

    Metadata {
        name: child_text(root, "name").or_else(|| child_text(root, "artifactId")),
        version: child_text(root, "version")
            .or_else(|| child(root, "parent").and_then(|parent| child_text(parent, "version"))),
        license: child(root, "licenses")
            .and_then(|licenses| child(licenses, "license"))
            .and_then(|license| child_text(license, "name")),
        edition: None,
    }
}

pub fn dotnet(contents: &str) -> Metadata {
    let Ok(document) = Document::parse(contents) else {
        return Metadata::default();
    };
    let root = document.root_element();

    Metadata {
        name: descendant_text(root, &["AssemblyName", "PackageId"]),
        version: descendant_text(root, &["Version", "VersionPrefix"]),
        license: descendant_text(root, &["PackageLicenseExpression"]),
        edition: None,
    }
}

fn cargo_value(document: &Value, key: &str) -> Option<String> {
    let package = document.get("package")?;

    if let Some(value) = string_at(Some(package), key) {
        return Some(value);
    }

    let inherited = package
        .get(key)
        .and_then(Value::as_table)
        .and_then(|settings| settings.get("workspace"))
        .and_then(Value::as_bool)
        .unwrap_or(false);

    if !inherited {
        return None;
    }

    document
        .get("workspace")
        .and_then(|workspace| workspace.get("package"))
        .and_then(|package| package.get(key))
        .and_then(value_to_string)
}

fn string_at(parent: Option<&Value>, key: &str) -> Option<String> {
    parent
        .and_then(|value| value.get(key))
        .and_then(value_to_string)
}

fn value_to_string(value: &Value) -> Option<String> {
    match value {
        Value::String(value) => Some(value.to_owned()),
        Value::Integer(value) => Some(value.to_string()),
        _ => None,
    }
}

fn json_string(document: &serde_json::Value, key: &str) -> Option<String> {
    document.get(key).and_then(|value| match value {
        serde_json::Value::String(value) => Some(value.to_owned()),
        serde_json::Value::Number(value) => Some(value.to_string()),
        _ => None,
    })
}

fn license_at(project: Option<&Value>) -> Option<String> {
    let license = project?.get("license")?;

    value_to_string(license).or_else(|| {
        license
            .get("text")
            .or_else(|| license.get("file"))
            .and_then(value_to_string)
    })
}

fn module_name(module: &str) -> Option<String> {
    module
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(str::to_owned)
}

fn child<'a, 'input>(node: Node<'a, 'input>, name: &str) -> Option<Node<'a, 'input>> {
    node.children()
        .find(|child| child.is_element() && child.tag_name().name() == name)
}

fn child_text(node: Node<'_, '_>, name: &str) -> Option<String> {
    child(node, name).and_then(node_text)
}

fn descendant_text(node: Node<'_, '_>, names: &[&str]) -> Option<String> {
    names.iter().find_map(|name| {
        node.descendants()
            .find(|child| child.is_element() && child.tag_name().name() == *name)
            .and_then(node_text)
    })
}

fn node_text(node: Node<'_, '_>) -> Option<String> {
    node.text()
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .map(str::to_owned)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cargo_reads_package_values_and_workspace_inheritance() {
        let metadata = cargo(
            r#"
[workspace.package]
version = "1.2.3"
edition = "2024"
license = "MIT"

[package]
name = 'demo'
version.workspace = true
edition.workspace = true
license.workspace = true
"#,
        );

        assert_eq!(metadata.name.as_deref(), Some("demo"));
        assert_eq!(metadata.version.as_deref(), Some("1.2.3"));
        assert_eq!(metadata.edition.as_deref(), Some("2024"));
        assert_eq!(metadata.license.as_deref(), Some("MIT"));
    }

    #[test]
    fn pyproject_supports_pep_621_and_poetry_metadata() {
        let pep = pyproject(
            r#"
[project]
name = "pep-demo"
version = "2.0.0"
license = { text = "Apache-2.0" }
"#,
        );
        let poetry = pyproject(
            r#"
[tool.poetry]
name = "poetry-demo"
version = "3.0.0"
license = "BSD-3-Clause"
"#,
        );

        assert_eq!(pep.name.as_deref(), Some("pep-demo"));
        assert_eq!(pep.license.as_deref(), Some("Apache-2.0"));
        assert_eq!(poetry.name.as_deref(), Some("poetry-demo"));
        assert_eq!(poetry.version.as_deref(), Some("3.0.0"));
    }

    #[test]
    fn reads_package_json_metadata() {
        let metadata = package_json(r#"{"name":"web-demo","version":"1.0.0","license":"MIT"}"#);

        assert_eq!(metadata.name.as_deref(), Some("web-demo"));
        assert_eq!(metadata.version.as_deref(), Some("1.0.0"));
        assert_eq!(metadata.license.as_deref(), Some("MIT"));
    }

    #[test]
    fn reads_go_maven_and_dotnet_metadata() {
        let go = go_mod("module github.com/example/rocket\n\ngo 1.24\n");
        let maven = maven(
            r#"<project><modelVersion>4.0.0</modelVersion><artifactId>demo-api</artifactId><version>1.4.0</version></project>"#,
        );
        let dotnet = dotnet(
            r#"<Project><PropertyGroup><AssemblyName>Demo.App</AssemblyName><Version>5.1.0</Version></PropertyGroup></Project>"#,
        );

        assert_eq!(go.name.as_deref(), Some("rocket"));
        assert_eq!(maven.name.as_deref(), Some("demo-api"));
        assert_eq!(maven.version.as_deref(), Some("1.4.0"));
        assert_eq!(dotnet.name.as_deref(), Some("Demo.App"));
        assert_eq!(dotnet.version.as_deref(), Some("5.1.0"));
    }
}
