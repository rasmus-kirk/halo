use super::utils::{CommitData, Scalar};
use halo_accumulation::pcdl::EvalProof;
use std::fmt::Display;

/// utility to print a CommitData for SnarkProof printing
fn print_comm(comm: CommitData) -> String {
    let mut result = String::from("{\n");
    result.push_str(&format!("    d: {},\n", comm.d));
    result.push_str(&format!("    x: {},\n", comm.pt.x));
    result.push_str(&format!("    y: {},\n", comm.pt.y));
    result.push_str(&format!("    z: {},\n", comm.pt.z));
    result.push_str("  }");
    result
}

/// Payload for the Verifier
#[derive(Debug)]
pub struct SnarkProof {
    pub a_xi: Scalar,
    pub a_com: CommitData,
    pub a_pi: EvalProof,
    pub b_xi: Scalar,
    pub b_com: CommitData,
    pub b_pi: EvalProof,
    pub c_xi: Scalar,
    pub c_com: CommitData,
    pub c_pi: EvalProof,
    pub f_gc_pi: EvalProof,

    pub z_xi: Scalar,
    pub z_com: CommitData,
    pub z_pi: EvalProof,
    pub zw_xi: Scalar,
    pub zw_pi: EvalProof,

    pub t_xi: Scalar,
    pub t_com: CommitData,
    pub t_pi: EvalProof,
    pub fp_xi: Scalar,
    pub fp_pi: EvalProof,
    pub fp_com: CommitData,
    pub gp_xi: Scalar,
    pub gp_pi: EvalProof,
    pub gp_com: CommitData,
}

impl Display for SnarkProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::from("πSNARK {\n");
        result.push_str(&format!("  [a(x)] {},\n", print_comm(self.a_com)));
        result.push_str(&format!("  [b(x)] {},\n", print_comm(self.b_com)));
        result.push_str(&format!("  [c(x)] {},\n", print_comm(self.c_com)));
        result.push_str(&format!("  [t_cc(x)] {},\n", print_comm(self.t_com)));
        result.push_str(&format!("  [z(x)] {},\n", print_comm(self.z_com)));
        result.push_str(&format!("  a(𝔷): {},\n", self.a_xi));
        result.push_str(&format!("  b(𝔷): {},\n", self.b_xi));
        result.push_str(&format!("  c(𝔷): {},\n", self.c_xi));
        result.push_str(&format!("  t_cc(𝔷): {},\n", self.t_xi));
        result.push_str(&format!("  z(𝔷): {},\n", self.z_xi));
        result.push_str(&format!("  z(ω𝔷): {},\n", self.zw_xi));
        result.push('}');
        writeln!(f, "{}", result)
    }
}

impl SnarkProof {
    pub fn print(&self) {
        println!("{}", self);
    }
}
