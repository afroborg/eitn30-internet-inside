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
            rule.push_str(&format!(" -i {}", in_interface));
        }

        if let Some(out_interface) = &self.out_interface {
            rule.push_str(&format!(" -o {}", out_interface));
        }

        if let Some(jump) = &self.jump {
            rule.push_str(&format!(" -j {}", jump));
        }

        if let Some(matching) = &self.matching {
            rule.push_str(&format!(" -m {}", matching));
        }

        if let Some(state) = &self.state {
            rule.push_str(&format!(" --state {}", state));
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
        let ipt = iptables::new(false).expect("Failed to initialize iptables");

        let rule = self.rule();

        ipt.delete(&self.table, &self.chain, &rule)
            .expect("Failed to delete rule");
    }
}
