#[derive(Default, Debug, Clone)]
pub struct IpTableEntry {
    in_iterface: Option<String>,
    out_interface: Option<String>,
    chain: String,
    table: String,
    jump: Option<String>,
    matching: Option<String>,
    state: Option<String>,
}

impl IpTableEntry {
    pub fn new(table: &str, chain: &str) -> Self {
        Self {
            table: table.to_owned(),
            chain: chain.to_owned(),
            ..Default::default()
        }
    }

    pub fn in_iterface(&mut self, in_iterface: &str) -> &mut Self {
        self.in_iterface = Some(in_iterface.to_owned());
        self
    }

    pub fn out_interface(&mut self, out_interface: &str) -> &mut Self {
        self.out_interface = Some(out_interface.to_owned());
        self
    }

    pub fn jump(&mut self, jump: &str) -> &mut Self {
        self.jump = Some(jump.to_owned());
        self
    }

    pub fn matching(&mut self, matching: &str) -> &mut Self {
        self.matching = Some(matching.to_owned());
        self
    }

    pub fn state(&mut self, state: &str) -> &mut Self {
        self.state = Some(state.to_owned());
        self
    }

    fn rule(&self) -> String {
        let mut rule = String::new();

        if let Some(in_interface) = &self.in_iterface {
            rule.push_str(&format!(" -i {in_interface}"));
        }

        if let Some(out_interface) = &self.out_interface {
            rule.push_str(&format!(" -o {out_interface}"));
        }

        if let Some(jump) = &self.jump {
            rule.push_str(&format!(" -j {jump}"));
        }

        if let Some(matching) = &self.matching {
            rule.push_str(&format!(" -m {matching}"));
        }

        if let Some(state) = &self.state {
            rule.push_str(&format!(" --state {state}"));
        }

        rule
    }

    pub fn apply(&self) -> Self {
        let ipt = iptables::new(false).expect("Failed to initialize iptables");

        let rule = self.rule();

        ipt.append(&self.table, &self.chain, &rule)
            .expect("Failed to append rule");

        self.clone()
    }

    pub fn drop(&self) {
        let Ok(ipt) = iptables::new(false) else {
            println!("Failed to drop iptables");
            return;
        };

        let rule = self.rule();

        ipt.delete(&self.table, &self.chain, &rule)
            .expect("Failed to delete rule");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let entry = IpTableEntry::new("filter", "INPUT");

        assert_eq!(entry.table, "filter");
        assert_eq!(entry.chain, "INPUT");
    }

    #[test]
    fn test_in_interface() {
        let mut entry = IpTableEntry::new("filter", "INPUT");

        entry.in_iterface("eth0");

        assert_eq!(entry.in_iterface, Some("eth0".to_owned()));
    }

    #[test]
    fn test_out_interface() {
        let mut entry = IpTableEntry::new("filter", "INPUT");

        entry.out_interface("eth0");

        assert_eq!(entry.out_interface, Some("eth0".to_owned()));
    }

    #[test]
    fn test_jump() {
        let mut entry = IpTableEntry::new("filter", "INPUT");

        entry.jump("ACCEPT");

        assert_eq!(entry.jump, Some("ACCEPT".to_owned()));
    }

    #[test]
    fn test_matching() {
        let mut entry = IpTableEntry::new("filter", "INPUT");

        entry.matching("tcp");

        assert_eq!(entry.matching, Some("tcp".to_owned()));
    }

    #[test]
    fn test_state() {
        let mut entry = IpTableEntry::new("filter", "INPUT");

        entry.state("NEW");

        assert_eq!(entry.state, Some("NEW".to_owned()));
    }

    #[test]
    fn test_rule() {
        let mut entry = IpTableEntry::new("filter", "INPUT");

        entry
            .in_iterface("eth0")
            .out_interface("eth1")
            .jump("ACCEPT")
            .matching("tcp")
            .state("NEW");

        assert_eq!(
            entry.rule(),
            " -i eth0 -o eth1 -j ACCEPT -m tcp --state NEW"
        );
    }
}
