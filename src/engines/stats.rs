

#[derive(Default)]
pub struct Stats {
    pub killer_hit0: u64,
    pub killer_hit1: u64,
    pub killer_updates: u64,
    pub killer_cutoffs: u64,      // cutoff caused by a move that was already a killer
    pub quiet_beta_cutoffs: u64,  // any quiet that causes beta cutoff
}

impl Stats {
    pub fn print(&self) {
        println!("── Killer Move Stats ───────────────────────────────");
        println!("Killer[0] hits     : {:>8}", self.killer_hit0);
        println!("Killer[1] hits     : {:>8}", self.killer_hit1);
        println!("Total killer hits  : {:>8}", self.killer_hit0 + self.killer_hit1);
        println!("Killer updates     : {:>8}", self.killer_updates);
        println!("Quiet β cutoffs    : {:>8}", self.quiet_beta_cutoffs);
        println!("Cutoffs from killer: {:>8}", self.killer_cutoffs);

        if self.quiet_beta_cutoffs > 0 {
            let pct = (self.killer_cutoffs as f64 / self.quiet_beta_cutoffs as f64) * 100.0;
            println!("Pct cutoffs from killer moves: {:>5.1}%", pct);
        }

        if self.killer_hit0 + self.killer_hit1 > 0 {
            let hit_ratio0 = (self.killer_hit0 as f64 / (self.killer_hit0 + self.killer_hit1) as f64) * 100.0;
            let hit_ratio1 = 100.0 - hit_ratio0;
            println!("Hit ratio K0:K1                  {:>4.1}% : {:>4.1}%", hit_ratio0, hit_ratio1);
        }
        println!("────────────────────────────────────────────────────");
    }
}
