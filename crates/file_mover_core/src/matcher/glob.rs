use crate::config::Rule;
use globset::{Glob, GlobSet, GlobSetBuilder};

pub struct CompiledRule {
    pub whitelist: Option<GlobSet>,
    pub blacklist: Option<GlobSet>,
}

fn build_globset(patterns: &[String]) -> Option<GlobSet> {
    if patterns.is_empty() {
        return None;
    }

    let mut builder = GlobSetBuilder::new();

    for p in patterns {
        builder.add(Glob::new(p).ok()?);
    }

    builder.build().ok()
}

pub fn compile_rule(rule: &Rule) -> CompiledRule {
    CompiledRule {
        whitelist: build_globset(&rule.whitelist),
        blacklist: build_globset(&rule.blacklist),
    }
}
