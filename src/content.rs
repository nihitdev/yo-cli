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

pub fn greeting(name: &str) -> String {
    choose(GREETINGS, 0xA3).replace("{name}", name)
}

pub fn hydration_reminder() -> String {
    choose(HYDRATION_REMINDERS, 0xB7).to_owned()
}

pub fn random_index(length: usize, salt: u64) -> usize {
    debug_assert!(length > 0, "random_index requires a non-empty list");

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;

    let entropy = now ^ (u64::from(process::id()) << 16) ^ salt;
    (entropy as usize) % length
}

fn choose<'a>(items: &'a [&'a str], salt: u64) -> &'a str {
    items[random_index(items.len(), salt)]
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
    }

    #[test]
    fn random_index_stays_within_bounds() {
        assert!(random_index(3, 42) < 3);
    }
}
