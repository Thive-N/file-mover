use crate::config::Config;
use crate::config::Rule;

impl Config {
    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn delete_rule(&mut self, name: &str) -> bool {
        let before = self.rules.len();
        self.rules.retain(|r| r.name != name);
        before != self.rules.len()
    }

    pub fn get_rule_mut(&mut self, name: &str) -> Option<&mut Rule> {
        self.rules.iter_mut().find(|r| r.name == name)
    }
}
