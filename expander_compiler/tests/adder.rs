mod sha256_utils;
use gf2::GF2;
use rand::RngCore;
use sha256_utils::*;

use expander_compiler::{
    declare_circuit,
    frontend::{
        compile, compile_cross_layer, BasicAPI, CompileResult, CompileResultCrossLayer, Define,
        GF2Config, Variable, API,
    },
};

declare_circuit!(CrossLayerAdder {
    a: [Variable; 32],
    b: [Variable; 32],
    c: [Variable; 32],
});

impl Define<GF2Config> for CrossLayerAdder<Variable> {
    fn define(&self, api: &mut API<GF2Config>) {
        let c_target = add_crosslayer(api, self.a.to_vec(), self.b.to_vec());
        for i in 0..32 {
            api.assert_is_equal(self.c[i], c_target[i]);
        }
    }
}

#[test]
fn test_add_vanilla() {
    let mut rng = rand::thread_rng();

    let n_tests = 100;
    let mut assignments = vec![];
    for _ in 0..n_tests {
        let a = rng.next_u32();
        let b = rng.next_u32();
        let (c, _overflowed) = a.overflowing_add(b);

        let mut assignment = CrossLayerAdder::<GF2>::default();
        for i in 0..32 {
            assignment.a[i] = ((a >> i) & 1).into();
            assignment.b[i] = ((b >> i) & 1).into();
            assignment.c[i] = ((c >> i) & 1).into();
        }

        assignments.push(assignment);
    }

    // layered circuit
    let compile_result = compile(&CrossLayerAdder::default()).unwrap();
    let CompileResult {
        witness_solver,
        layered_circuit,
    } = compile_result;
    let witness = witness_solver.solve_witnesses(&assignments).unwrap();
    let res = layered_circuit.run(&witness);
    let expected_res = vec![true; n_tests];
    assert_eq!(res, expected_res);

    // crosslayer circuit
    let compile_result = compile_cross_layer(&CrossLayerAdder::default()).unwrap();
    let CompileResultCrossLayer {
        witness_solver,
        layered_circuit,
    } = compile_result;
    let witness = witness_solver.solve_witnesses(&assignments).unwrap();
    let res = layered_circuit.run(&witness);
    let expected_res = vec![true; n_tests];
    assert_eq!(res, expected_res);
}
