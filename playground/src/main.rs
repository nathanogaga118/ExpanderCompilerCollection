use expander_compiler::frontend::*;
use expander_compiler::{
    declare_circuit,
    frontend::{BN254Config, BasicAPI, Define, Variable, API},
};
use halo2curves::bn256::Fr;

declare_circuit!(ExampleCircuit {
    p3_poly: [Variable; 8],
    q3_poly: [Variable; 8],
    p1_poly: [Variable; 4],
});

fn custom_gate(
    builder: &mut API<BN254Config>,
    a: &Variable,
    b: &Variable,
    c: &Variable,
    d: &Variable,
) -> Variable {
    let t1 = builder.mul(a, b);
    let t2 = builder.mul(c, d);
    builder.add(&t1, &t2)
}

impl Define<BN254Config> for ExampleCircuit<Variable> {
    fn define(&self, builder: &mut API<BN254Config>) {
        let mut p2 = vec![];
        let mut p1_poly_in_circuit = vec![];
        for i in 0..8 {
            let other_index = if i & 1 == 0 { i + 1 } else { i - 1 };
            let res = custom_gate(
                builder,
                &self.p3_poly[i],
                &self.q3_poly[other_index],
                &self.p3_poly[other_index],
                &self.q3_poly[i],
            );
            p2.push(res);
        }

        p1_poly_in_circuit.push(builder.add(&p2[0], &p2[2]));
        p1_poly_in_circuit.push(builder.add(&p2[1], &p2[3]));
        p1_poly_in_circuit.push(builder.add(&p2[4], &p2[5]));
        p1_poly_in_circuit.push(builder.add(&p2[6], &p2[7]));

        for i in 0..4 {
            builder.assert_is_equal(&self.p1_poly[i], &p1_poly_in_circuit[i]);
        }
    }
}

impl ExampleCircuit<Fr> {
    fn new(p3_poly: [u32; 8], q3_poly: [u32; 8]) -> Self {
        let p3_poly = p3_poly.iter().map(|x| Fr::from(*x)).collect::<Vec<Fr>>();
        let q3_poly = q3_poly.iter().map(|x| Fr::from(*x)).collect::<Vec<Fr>>();

        let mut p2 = [Fr::zero(); 8];
        let mut p1_poly = [Fr::zero(); 4];
        for i in 0..8 {
            let other_index = if i & 1 == 0 { i + 1 } else { i - 1 };
            let t1 = p3_poly[i] * q3_poly[other_index];
            let t2 = p3_poly[other_index] * q3_poly[i];
            p2[i] = t1 + t2;
        }

        p1_poly[0] = p2[0] + p2[2];
        p1_poly[1] = p2[1] + p2[3];
        p1_poly[2] = p2[4] + p2[5];
        p1_poly[3] = p2[6] + p2[7];

        ExampleCircuit {
            p3_poly: p3_poly.try_into().unwrap(),
            q3_poly: q3_poly.try_into().unwrap(),
            p1_poly,
        }
    }
}

fn main() {
    // build a dummy circuit
    let compile_result = compile(&ExampleCircuit::default()).unwrap();

    let p3_poly = [1, 2, 3, 4, 5, 6, 7, 8];
    let q3_poly = [8, 7, 6, 5, 4, 3, 2, 1];
    let assignment = ExampleCircuit::new(p3_poly, q3_poly);

    // check the witnesses are correct
    let witness = compile_result
        .witness_solver
        .solve_witness(&assignment)
        .unwrap();

    let output = compile_result.layered_circuit.run(&witness);

    assert_eq!(output, vec![true]);
}
