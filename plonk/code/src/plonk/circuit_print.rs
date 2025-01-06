use super::{
    circuit_abstract::{Gate, WireId},
    circuit_protocol::Circuit,
};
use ascii_table::{Align, AsciiTable};
use halo_accumulation::group::PallasScalar;

pub fn map_to_alphabet(mut num: usize) -> String {
    let mut result = String::new();
    num += 1; // To make it 1-based index
    while num > 0 {
        num -= 1;
        result.push((b'a' + (num % 26) as u8) as char);
        num /= 26;
    }
    result.push('_');
    result.chars().rev().collect()
}

fn to_eqn_str(a_: WireId, b_: WireId, c_: WireId, is_add: bool) -> String {
    let op = if is_add { "+" } else { "⨉" };
    let a = map_to_alphabet(a_);
    let b = map_to_alphabet(b_);
    let c = map_to_alphabet(c_);
    format!("{} {} {} = {}", a, op, b, c)
}

pub fn scalar_to_str(val: PallasScalar) -> String {
    let v1 = val.to_string();
    let v2 = (-val).to_string();
    if v1.len() <= v2.len() {
        v1
    } else {
        format!("-{}", v2)
    }
}

fn xpoint_to_str(coord: (usize, usize)) -> String {
    let col = match coord.0 {
        0 => "a",
        1 => "b",
        2 => "c",
        _ => "m",
    };
    format!("{}({})", col, coord.1)
}

impl<const L: usize> Circuit<L> {
    fn to_polynomial_const_data(&self, row: usize, x_: WireId) -> Vec<String> {
        let b = map_to_alphabet(x_);
        vec![
            format!("{}:{}", b, scalar_to_str(self.evals[0][row])),
            scalar_to_str(self.evals[1][row]),
            scalar_to_str(self.evals[2][row]),
            scalar_to_str(self.evals[3][row]),
            scalar_to_str(self.evals[4][row]),
            scalar_to_str(self.evals[5][row]),
            scalar_to_str(self.evals[6][row]),
            scalar_to_str(self.evals[7][row]),
        ]
    }

    fn to_polynomial_data(&self, row: usize, a_: WireId, b_: WireId, c_: WireId) -> Vec<String> {
        let a = format!(
            "{}:{}",
            map_to_alphabet(a_),
            scalar_to_str(self.evals[0][row])
        );
        let b = format!(
            "{}:{}",
            map_to_alphabet(b_),
            scalar_to_str(self.evals[1][row])
        );
        let c = format!(
            "{}:{}",
            map_to_alphabet(c_),
            scalar_to_str(self.evals[2][row])
        );
        let q_a = scalar_to_str(self.evals[3][row]);
        let q_b = scalar_to_str(self.evals[4][row]);
        let q_o = scalar_to_str(self.evals[5][row]);
        let q_m = scalar_to_str(self.evals[6][row]);
        let q_c = scalar_to_str(self.evals[7][row]);
        vec![a, b, c, q_a, q_b, q_o, q_m, q_c]
    }

    pub fn print_circuit_info(&self) {
        let mut input_wires_str: Vec<String> = Vec::new();
        for wire in 0..L {
            input_wires_str.push(format!(
                "  {} = {}",
                map_to_alphabet(wire),
                self.evals[0][wire]
            ));
        }
        println!("Input Wires:\n{}", input_wires_str.join("\n"));

        let mut const_wires_str: Vec<String> = Vec::new();
        for (val, wire) in self.consts.iter() {
            const_wires_str.push(format!(
                "  {} = {}",
                map_to_alphabet(*wire),
                scalar_to_str(*val)
            ));
        }
        println!("\nConstant Wires:\n{}", const_wires_str.join("\n"));

        println!("\nGates:");
        for (Gate { l, r, is_add }, o) in self.gates.iter() {
            println!("  {}", to_eqn_str(*l, *r, *o, *is_add));
        }

        println!("\na(x)ql(x) + b(x)qr(x) + c(x)qo(x) + l(x)r(x)qm(x) + qc(x) = 0");
        let mut ascii_table = AsciiTable::default();
        let headers = ["i", "a", "b", "c", "ql", "qr", "qo", "qm", "qc"];
        for (i, header) in headers.iter().enumerate() {
            ascii_table
                .column(i)
                .set_header(*header)
                .set_align(Align::Left);
        }
        let mut data: Vec<Vec<String>> = Vec::new();
        for i in 0..L {
            data.push(vec![
                i.to_string(),
                format!("{}:{}", map_to_alphabet(i), scalar_to_str(self.evals[0][i])),
                format!("{}", scalar_to_str(self.evals[1][i])),
                format!("{}", scalar_to_str(self.evals[2][i])),
                format!("{}", scalar_to_str(self.evals[3][i])),
                format!("{}", scalar_to_str(self.evals[4][i])),
                format!("{}", scalar_to_str(self.evals[5][i])),
                format!("{}", scalar_to_str(self.evals[6][i])),
                format!("{}", scalar_to_str(self.evals[7][i])),
            ]);
        }
        for (i, &(_, wire)) in self.consts.iter().enumerate() {
            let row = self.to_polynomial_const_data(L + i, wire);
            let mut row_with_index = vec![(data.len()).to_string()];
            row_with_index.extend(row);
            data.push(row_with_index);
        }
        let cl = self.consts.len();
        for (i, &(Gate { l, r, is_add: _ }, o)) in self.gates.iter().enumerate() {
            let row = self.to_polynomial_data(L + cl + i, l, r, o);
            let mut row_with_index = vec![(data.len()).to_string()];
            row_with_index.extend(row);
            data.push(row_with_index);
        }

        ascii_table.print(data);

        println!("\nRoots of Unity:\n  ω^{} = 1", self.order);

        println!("\nConstraints:");
        for (i, constraint) in self.classes.iter().enumerate() {
            if constraint.len() == 1 {
                continue;
            }
            let wire_str: Vec<String> = constraint.iter().map(|&x| xpoint_to_str(x)).collect();
            println!("  {} = {}", map_to_alphabet(i), wire_str.join(", "))
        }
        println!();
    }
}
