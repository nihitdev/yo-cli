use std::{
    process,
    time::{SystemTime, UNIX_EPOCH},
};

const GREETINGS: &[&str] = &[
    "YOOOOO, {name}! 🔥",
    "What are we shipping today, {name}? 🚀",
    "Terminal open. Brain online. Let's go, {name}. ⚡",
    "Welcome back, {name}. Make something awesome. 🦀",
    "Time to build, {name}. No excuses. 💻",
];

const HYDRATION_REMINDERS: &[&str] = &[
    "Drink some water before the bug drinks your sanity.",
    "Hydration check: take a sip.",
    "Water first. Then infinite debugging power.",
];

const TIPS: &[&str] = &[
    "Commit small, meaningful changes before starting the next feature.",
    "A tiny reproducible bug report beats a vague error description.",
    "Write the test that would have caught your last bug.",
    "When stuck, reduce the problem until it looks almost silly.",
    "Make it work, make it clear, then make it fast.",
    "A clean README is part of the project, not an afterthought.",
];

pub fn greeting(name: &str) -> String {
    choose(GREETINGS, 0xA3).replace("{name}", name)
}

pub fn hydration_reminder() -> String {
    choose(HYDRATION_REMINDERS, 0xB7).to_owned()
}

pub fn tip() -> String {
    choose(TIPS, 0xC1).to_owned()
}

fn choose<'a>(items: &'a [&'a str], salt: u64) -> &'a str {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;

    let entropy = now ^ (u64::from(process::id()) << 16) ^ salt;
    let index = (entropy as usize) % items.len();
    items[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        assert!(greeting("Nihit").contains("Nihit"));
    }

    #[test]
    fn content_is_never_empty() {
        assert!(!hydration_reminder().is_empty());
        assert!(!tip().is_empty());
    }
}
