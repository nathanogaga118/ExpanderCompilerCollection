use crate::gnark::emparam::FieldParams;
use crate::gnark::element::*;
use crate::gnark::field::Field as GField;
use crate::gnark::emparam::*;
use crate::gnark::hints::*;
use std::collections::HashMap;
use std::hint;
use crate::logup::*;
use expander_compiler::frontend::extra::*;
use expander_compiler::{circuit::layered::InputType, frontend::*};
use num_bigint::BigInt;

use super::e2::*;
#[derive(Default, Clone)]
pub struct GE6 {
    pub b0: GE2,
    pub b1: GE2,
    pub b2: GE2,
}
impl GE6 {
    pub fn clone(&self) -> Self {
        GE6 {
            b0: self.b0.clone(),
            b1: self.b1.clone(),
            b2: self.b2.clone(),
        }
    }
}
pub struct Ext6 {
    pub ext2: Ext2,
}

impl Ext6{
    pub fn new<'a, C:Config, B:RootAPI<C>>(api: &'a mut B) -> Self {
        Self {
            ext2: Ext2::new(api),
        }
    }
    pub fn one(&mut self) -> GE6 {
        let b0 = self.ext2.one();
        let b1 = self.ext2.zero();
        let b2 = self.ext2.zero();
        GE6 {
            b0,
            b1,
            b2
        }
    }
    pub fn zero<'a, C:Config, B:RootAPI<C>>(&mut self) -> GE6 {
        let b0 = self.ext2.zero();
        let b1 = self.ext2.zero();
        let b2 = self.ext2.zero();
        GE6 {
            b0,
            b1,
            b2
        }
    }
    pub fn is_zero<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, z: &GE6) -> Variable {
        let b0 = self.ext2.is_zero(native, &z.b0.clone());
        let b1 = self.ext2.is_zero(native, &z.b1.clone());
        let b2 = self.ext2.is_zero(native, &z.b2.clone());
        let tmp = native.and(b0, b1);
        native.and(tmp, b2)
    }
    pub fn add<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE6, y: &GE6) -> GE6 {
        let z0 = self.ext2.add(native, &x.b0.clone(), &y.b0.clone());
        let z1 = self.ext2.add(native, &x.b1.clone(), &y.b1.clone());
        let z2 = self.ext2.add(native, &x.b2.clone(), &y.b2.clone());
        GE6 {
            b0: z0,
            b1: z1,
            b2: z2,
        }
    }
    pub fn neg<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE6) -> GE6 {
        let z0 = self.ext2.neg(native, &x.b0.clone());
        let z1 = self.ext2.neg(native, &x.b1.clone());
        let z2 = self.ext2.neg(native, &x.b2.clone());
        GE6 {
            b0: z0,
            b1: z1,
            b2: z2,
        }
    }
    pub fn sub<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE6, y: &GE6) -> GE6 {
        let z0 = self.ext2.sub(native, &x.b0.clone(), &y.b0.clone());
        let z1 = self.ext2.sub(native, &x.b1.clone(), &y.b1.clone());
        let z2 = self.ext2.sub(native, &x.b2.clone(), &y.b2.clone());
        GE6 {
            b0: z0,
            b1: z1,
            b2: z2,
        }
    }
    pub fn double<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE6) -> GE6 {
        let z0 = self.ext2.double(native, &x.b0.clone());
        let z1 = self.ext2.double(native, &x.b1.clone());
        let z2 = self.ext2.double(native, &x.b2.clone());
        GE6 {
            b0: z0,
            b1: z1,
            b2: z2,
        }
    }
    pub fn square<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE6) -> GE6 {
        let c4 = self.ext2.mul(native, &x.b0.clone(), &x.b1.clone());
        let c4 = self.ext2.double(native, &c4);
        let c5 = self.ext2.square(native, &x.b2.clone());
        let c1 = self.ext2.mul_by_non_residue(native, &c5);
        let c1 = self.ext2.add(native, &c1, &c4);
        let c2 = self.ext2.sub(native, &c4, &c5);
        let c3 = self.ext2.square(native, &x.b0.clone());
        let c4 = self.ext2.sub(native, &x.b0.clone(), &x.b1.clone());
        let c4 = self.ext2.add(native, &c4, &x.b2.clone());
        let c5 = self.ext2.mul(native, &x.b1.clone(), &x.b2.clone());
        let c5 = self.ext2.double(native, &c5);
        let c4 = self.ext2.square(native, &c4);
        let c0 = self.ext2.mul_by_non_residue(native, &c5);
        let c0 = self.ext2.add(native, &c0, &c3);
        let z2 = self.ext2.add(native, &c2, &c4);
        let z2 = self.ext2.add(native, &z2, &c5);
        let z2 = self.ext2.sub(native, &z2, &c3);
        let z0 = c0;
        let z1 = c1;
        GE6 {
            b0: z0,
            b1: z1,
            b2: z2,
        }
    }
    pub fn mul_by_e2<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE6, y: &GE2) -> GE6 {
        let z0 = self.ext2.mul(native, &x.b0.clone(), y);
        let z1 = self.ext2.mul(native, &x.b1.clone(), y);
        let z2 = self.ext2.mul(native, &x.b2.clone(), y);
        GE6 {
            b0: z0,
            b1: z1,
            b2: z2,
        }
    }
    pub fn mul_by_12<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE6, b1: &GE2, b2: &GE2) -> GE6 {
        let t1 = self.ext2.mul(native, &x.b1.clone(), b1);
        let t2 = self.ext2.mul(native, &x.b2.clone(), b2);
        let mut c0 = self.ext2.add(native, &x.b1.clone(), &x.b2.clone());
        let mut tmp = self.ext2.add(native, b1, b2);
        c0 = self.ext2.mul(native, &c0, &tmp);
        tmp = self.ext2.add(native, &t1, &t2);
        c0 = self.ext2.sub(native, &c0, &tmp);
        c0 = self.ext2.mul_by_non_residue(native, &c0);
        let mut c1 = self.ext2.add(native, &x.b0.clone(), &x.b1.clone());
        c1 = self.ext2.mul(native, &c1, b1);
        c1 = self.ext2.sub(native, &c1, &t1);
        tmp = self.ext2.mul_by_non_residue(native, &t2);
        c1 = self.ext2.add(native, &c1, &tmp);
        tmp = self.ext2.add(native, &x.b0.clone(), &x.b2.clone());
        let mut c2 = self.ext2.mul(native, b2, &tmp);
        c2 = self.ext2.sub(native, &c2, &t2);
        c2 = self.ext2.add(native, &c2, &t1);
        GE6 {
            b0: c0,
            b1: c1,
            b2: c2,
        }
    }
    pub fn mul_by_0<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, z: &GE6, c0: &GE2) -> GE6 {
        let a = self.ext2.mul(native, &z.b0.clone(), c0);
        let tmp = self.ext2.add(native, &z.b0.clone(), &z.b2.clone());
        let mut t2 = self.ext2.mul(native, c0, &tmp);
        t2 = self.ext2.sub(native, &t2, &a);
        let tmp = self.ext2.add(native, &z.b0.clone(), &z.b1.clone());
        let mut t1 = self.ext2.mul(native, c0, &tmp);
        t1 = self.ext2.sub(native, &t1, &a);
        GE6 {
            b0: a,
            b1: t1,
            b2: t2,
        }
    }
    pub fn mul_by_01<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, z: &GE6, c0: &GE2, c1: &GE2) -> GE6 {
        let a = self.ext2.mul(native, &z.b0, c0);
        let b = self.ext2.mul(native, &z.b1, c1);
        let tmp = self.ext2.add(native, &z.b1.clone(), &z.b2.clone());
        let mut t0 = self.ext2.mul(native, c1, &tmp);

        // println!("t0");
        // print_e2(native, &t0);
        t0 = self.ext2.sub(native, &t0, &b);
        // println!("t01");
        // print_e2(native, &t0);
        t0 = self.ext2.mul_by_non_residue(native, &t0);
        // println!("t02");
        // print_e2(native, &t0);
        t0 = self.ext2.add(native, &t0, &a);
        // println!("t03");
        // print_e2(native, &t0);
        let mut t2 = self.ext2.mul(native, &z.b2.clone(), c0);
        t2 = self.ext2.add(native, &t2, &b);
        let mut t1 = self.ext2.add(native, c0, c1);
        let tmp = self.ext2.add(native, &z.b0.clone(), &z.b1.clone());
        t1 = self.ext2.mul(native, &t1, &tmp);
        let tmp = self.ext2.add(native, &a, &b);
        t1 = self.ext2.sub(native, &t1, &tmp);
        GE6 {
            b0: t0,
            b1: t1,
            b2: t2,
        }
    }
    pub fn mul_by_non_residue<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE6) -> GE6 {
        let z0 = self.ext2.mul_by_non_residue(native, &x.b2.clone());
        GE6 {
            b0: z0,
            b1: x.b0.clone(),
            b2: x.b1.clone(),
        }
    }
    pub fn assert_isequal<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE6, y: &GE6) {
        self.ext2.assert_isequal(native, &x.b0, &y.b0);
        self.ext2.assert_isequal(native, &x.b1, &y.b1);
        self.ext2.assert_isequal(native, &x.b2, &y.b2);
    }
    pub fn select<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, selector: Variable, z1: &GE6, z0: &GE6) -> GE6 {
        let b0 = self.ext2.select(native, selector, &z1.b0.clone(), &z0.b0.clone());
        let b1 = self.ext2.select(native, selector, &z1.b1.clone(), &z0.b1.clone());
        let b2 = self.ext2.select(native, selector, &z1.b2.clone(), &z0.b2.clone());
        GE6 {
            b0: b0,
            b1: b1,
            b2: b2,
        }
    }
    pub fn mul_karatsuba_over_karatsuba<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE6, y: &GE6) -> GE6 {
        let t0 = self.ext2.mul(native, &x.b0.clone(), &y.b0.clone());
        let t1 = self.ext2.mul(native, &x.b1.clone(), &y.b1.clone());
        let t2 = self.ext2.mul(native, &x.b2.clone(), &y.b2.clone());
        let mut c0 = self.ext2.add(native, &x.b1.clone(), &x.b2.clone());
        let mut tmp = self.ext2.add(native, &y.b1.clone(), &y.b2.clone());
        c0 = self.ext2.mul(native, &c0, &tmp);
        tmp = self.ext2.add(native, &t2, &t1);
        c0 = self.ext2.sub(native, &c0, &tmp);
        c0 = self.ext2.mul_by_non_residue(native, &c0);
        c0 = self.ext2.add(native, &c0, &t0);
        let mut c1 = self.ext2.add(native, &x.b0.clone(), &x.b1.clone());
        tmp = self.ext2.add(native, &y.b0.clone(), &y.b1.clone());
        c1 = self.ext2.mul(native, &c1, &tmp);
        tmp = self.ext2.add(native, &t0, &t1);
        c1 = self.ext2.sub(native, &c1, &tmp);
        tmp = self.ext2.mul_by_non_residue(native, &t2);
        c1 = self.ext2.add(native, &c1, &tmp);
        let mut tmp = self.ext2.add(native, &x.b0.clone(), &x.b2.clone());
        let mut c2 = self.ext2.add(native, &y.b0.clone(), &y.b2.clone());
        // println!("special");
        // print_e2(native, &c2);
        // print_e2(native, &tmp);
        c2 = self.ext2.mul(native, &c2, &tmp);
        tmp = self.ext2.add(native, &t0, &t2);
        c2 = self.ext2.sub(native, &c2, &tmp);
        c2 = self.ext2.add(native, &c2, &t1);
        GE6 {
            b0: c0,
            b1: c1,
            b2: c2,
        }
    }
    pub fn mul<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE6, y: &GE6) -> GE6 {
        self.mul_karatsuba_over_karatsuba(native, x, y)
    }
    pub fn div<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE6, y: &GE6) -> GE6 {
        let inputs = vec![x.b0.a0.clone(), x.b0.a1.clone(), x.b1.a0.clone(), x.b1.a1.clone(), x.b2.a0.clone(), x.b2.a1.clone(), y.b0.a0.clone(), y.b0.a1.clone(), y.b1.a0.clone(), y.b1.a1.clone(), y.b2.a0.clone(), y.b2.a1.clone()];
        let output = self.ext2.fp.new_hint(native, "myhint.dive6hint", 6, inputs);
        let div = GE6 {
            b0: GE2 {
                a0: output[0].clone(),
                a1: output[1].clone(),
            },
            b1: GE2 {
                a0: output[2].clone(),
                a1: output[3].clone(),
            },
            b2: GE2 {
                a0: output[4].clone(),
                a1: output[5].clone(),
            },
        };
        // println!("div");
        // print_element(native, &div.b0.a0);
        // print_element(native, &div.b0.a1);
        // print_element(native, &div.b1.a0);
        // print_element(native, &div.b1.a1);
        // print_element(native, &div.b2.a0);
        // print_element(native, &div.b2.a1);
        let _x = self.mul(native, &div, y);
        // println!("_x");
        // print_element(native, &_x.b0.a0);
        // print_element(native, &_x.b0.a1);
        // print_element(native, &_x.b1.a0);
        // print_element(native, &_x.b1.a1);
        // print_element(native, &_x.b2.a0);
        // print_element(native, &_x.b2.a1);
        self.assert_isequal(native, &x.clone(), &_x);
        div
    }
    pub fn inverse_div<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE6) -> GE6 {
        let one = self.one();
        self.div(native, &one, x)
    }
    pub fn inverse<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE6) -> GE6 {
        let inputs = vec![x.b0.a0.clone(), x.b0.a1.clone(), x.b1.a0.clone(), x.b1.a1.clone(), x.b2.a0.clone(), x.b2.a1.clone()];
        let output = self.ext2.fp.new_hint(native, "myhint.inversee6hint", 6, inputs);
        let inv = GE6 {
            b0: GE2 {
                a0: output[0].clone(),
                a1: output[1].clone(),
            },
            b1: GE2 {
                a0: output[2].clone(),
                a1: output[3].clone(),
            },
            b2: GE2 {
                a0: output[4].clone(),
                a1: output[5].clone(),
            },
        };
        let one = self.one();
        let _one = self.mul(native, &inv, x);
        self.assert_isequal(native, &one, &_one);
        inv
    }
    pub fn div_e6_by_6<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &[Element<bls12381_fp>; 6]) -> [Element<bls12381_fp>; 6] {
        let inputs = vec![x[0].clone(), x[1].clone(), x[2].clone(), x[3].clone(), x[4].clone(), x[5].clone()];
        let output = self.ext2.fp.new_hint(native, "myhint.dive6by6hint", 6, inputs);
        let y0 = output[0].clone();
        let y1 = output[1].clone();
        let y2 = output[2].clone();
        let y3 = output[3].clone();
        let y4 = output[4].clone();
        let y5 = output[5].clone();
        let x0 = self.ext2.fp.mul_const(native, &y0, BigInt::from(6));
        let x1 = self.ext2.fp.mul_const(native, &y1, BigInt::from(6));
        let x2 = self.ext2.fp.mul_const(native, &y2, BigInt::from(6));
        let x3 = self.ext2.fp.mul_const(native, &y3, BigInt::from(6));
        let x4 = self.ext2.fp.mul_const(native, &y4, BigInt::from(6));
        let x5 = self.ext2.fp.mul_const(native, &y5, BigInt::from(6));
        self.ext2.fp.assert_isequal(native, &x[0], &x0);
        self.ext2.fp.assert_isequal(native, &x[1], &x1);
        self.ext2.fp.assert_isequal(native, &x[2], &x2);
        self.ext2.fp.assert_isequal(native, &x[3], &x3);
        self.ext2.fp.assert_isequal(native, &x[4], &x4);
        self.ext2.fp.assert_isequal(native, &x[5], &x5);
        [y0, y1, y2, y3, y4, y5]
    }
}



declare_circuit!(E6AddCircuit {
    x: [[[Variable; 48];2];3],
    y: [[[Variable; 48];2];3],
    z: [[[Variable; 48];2];3],
});

impl GenericDefine<M31Config> for E6AddCircuit<Variable> {
    fn define<Builder: RootAPI<M31Config>>(&self, builder: &mut Builder) {
        let mut ext6 = Ext6::new(builder);
        let x_e6 = GE6 {
            b0: GE2 {
                a0: new_internal_element(self.x[0][0].to_vec(), 0),
                a1: new_internal_element(self.x[0][1].to_vec(), 0),
            },
            b1: GE2 {
                a0: new_internal_element(self.x[1][0].to_vec(), 0),
                a1: new_internal_element(self.x[1][1].to_vec(), 0),
            },
            b2: GE2 {
                a0: new_internal_element(self.x[2][0].to_vec(), 0),
                a1: new_internal_element(self.x[2][1].to_vec(), 0),
            },
        };
        let y_e6 = GE6 {
            b0: GE2 {
                a0: new_internal_element(self.y[0][0].to_vec(), 0),
                a1: new_internal_element(self.y[0][1].to_vec(), 0),
            },
            b1: GE2 {
                a0: new_internal_element(self.y[1][0].to_vec(), 0),
                a1: new_internal_element(self.y[1][1].to_vec(), 0),
            },
            b2: GE2 {
                a0: new_internal_element(self.y[2][0].to_vec(), 0),
                a1: new_internal_element(self.y[2][1].to_vec(), 0),
            },
        };
        let z_e6 = GE6 {
            b0: GE2 {
                a0: new_internal_element(self.z[0][0].to_vec(), 0),
                a1: new_internal_element(self.z[0][1].to_vec(), 0),
            },
            b1: GE2 {
                a0: new_internal_element(self.z[1][0].to_vec(), 0),
                a1: new_internal_element(self.z[1][1].to_vec(), 0),
            },
            b2: GE2 {
                a0: new_internal_element(self.z[2][0].to_vec(), 0),
                a1: new_internal_element(self.z[2][1].to_vec(), 0),
            },
        };
        let z = ext6.add(builder, &x_e6, &y_e6);
        ext6.assert_isequal(builder, &z, &z_e6);
        ext6.ext2.fp.check_mul(builder);
        ext6.ext2.fp.table.final_check(builder);
    }
}
#[test]
fn test_e6_add() {
    // let compile_result = compile(&E2AddCircuit::default()).unwrap();
    let compile_result =
    compile_generic(&E6AddCircuit::default(), CompileOptions::default()).unwrap();
	let mut hint_registry = HintRegistry::<M31>::new();
    register_hint(&mut hint_registry);
    let mut assignment = E6AddCircuit::<M31> {
        x: [[[M31::from(0); 48]; 2]; 3],
        y: [[[M31::from(0); 48]; 2]; 3],
        z: [[[M31::from(0); 48]; 2]; 3],
    };

    let x0_b0_a0_bytes = [43,211,155,220,85,4,8,1,215,211,93,215,81,21,56,57,139,64,114,222,34,249,133,1,89,193,221,30,159,24,10,156,26,94,220,176,241,186,246,191,181,92,117,198,20,54,44,14,];
    let x0_b0_a1_bytes = [63,131,211,85,212,40,216,174,142,150,21,245,183,100,255,199,209,21,209,87,66,192,97,175,236,116,95,238,93,20,154,35,164,253,56,202,205,64,0,200,179,17,69,28,185,161,70,13,];
    let x0_b1_a0_bytes = [214,62,78,148,85,33,95,146,49,88,94,54,52,208,3,136,177,46,77,253,17,128,131,235,82,176,80,134,59,52,163,238,32,181,131,56,17,55,66,102,145,191,18,175,151,1,212,23,];
    let x0_b1_a1_bytes = [41,167,64,159,223,51,189,43,186,251,202,72,55,36,85,193,232,226,132,96,154,82,119,118,133,141,95,19,205,2,134,48,181,178,133,101,88,189,43,189,238,133,161,60,82,210,193,25,];
    let x0_b2_a0_bytes = [69,152,0,136,208,43,221,129,150,113,46,202,33,249,218,176,47,123,129,203,88,135,65,235,24,13,135,20,230,253,169,246,55,229,221,139,91,205,100,77,117,152,144,112,64,105,19,21,];
    let x0_b2_a1_bytes = [91,154,129,212,234,209,169,160,142,49,247,206,85,255,156,123,218,140,13,35,79,130,173,36,205,226,38,38,253,40,49,195,138,58,160,15,228,18,97,149,42,224,34,135,225,42,216,15,];
    let x1_b0_a0_bytes = [168,144,97,71,250,233,57,194,117,19,227,238,182,173,56,31,77,42,237,203,81,157,105,108,51,186,234,114,230,161,213,26,154,32,89,75,11,160,27,146,90,226,1,45,226,94,235,23,];
    let x1_b0_a1_bytes = [1,241,173,149,51,212,21,36,198,72,155,117,227,230,43,12,239,110,117,76,151,134,20,75,136,2,197,149,210,100,232,213,66,182,114,49,237,192,134,188,192,157,229,5,205,26,72,7,];
    let x1_b1_a0_bytes = [5,131,227,108,57,93,117,63,62,3,235,177,236,31,181,189,212,89,138,143,76,255,243,255,18,170,199,28,241,228,251,200,4,18,141,186,170,58,136,235,114,55,39,38,1,16,35,1,];
    let x1_b1_a1_bytes = [125,64,186,137,111,34,155,104,156,45,242,173,235,118,208,41,134,62,54,225,33,126,182,34,254,7,92,226,214,219,134,153,38,192,67,164,136,69,162,207,122,195,73,43,24,120,96,13,];
    let x1_b2_a0_bytes = [145,182,101,27,67,208,10,14,239,224,162,122,20,230,25,90,124,227,52,206,100,13,49,213,210,224,63,236,90,227,56,138,35,218,165,113,114,120,139,135,191,21,32,64,126,59,230,2,];
    let x1_b2_a1_bytes = [93,163,83,188,82,139,106,196,217,193,42,85,147,98,114,220,131,93,17,61,214,81,211,13,80,49,149,41,98,183,38,215,179,227,251,194,75,197,11,128,111,231,95,246,179,151,8,10,];
    let x2_b0_a0_bytes = [40,185,253,35,80,238,66,9,77,231,236,20,10,195,196,57,180,116,174,179,211,195,190,6,205,104,67,158,0,111,104,82,221,209,233,184,70,179,246,6,118,88,247,185,12,131,22,12,];
    let x2_b0_a1_bytes = [64,116,129,235,7,253,237,210,84,223,176,106,155,75,43,212,192,132,70,164,217,70,118,250,116,119,36,132,48,121,130,249,230,179,171,251,186,1,135,132,116,175,42,34,134,188,142,20,];
    let x2_b1_a0_bytes = [219,193,49,1,143,126,212,209,111,91,73,232,32,240,184,69,134,136,215,140,94,127,119,235,101,90,24,163,44,25,159,183,37,199,16,243,187,113,202,81,4,247,57,213,152,17,247,24,];
    let x2_b1_a1_bytes = [251,60,251,40,79,86,89,218,86,41,105,69,36,155,121,204,74,43,10,75,27,254,252,49,196,130,54,2,31,147,149,101,4,198,125,198,42,91,178,65,207,98,107,46,128,56,33,13,];
    let x2_b2_a0_bytes = [214,78,102,163,19,252,231,143,133,82,209,68,54,223,244,10,172,94,182,153,189,148,114,192,235,237,198,0,65,225,226,128,91,191,131,253,205,69,240,212,52,174,176,176,190,164,249,23,];
    let x2_b2_a1_bytes = [184,61,213,144,61,93,20,101,104,243,33,36,233,97,15,88,94,234,30,96,37,212,128,50,29,20,188,79,95,224,87,154,62,30,156,210,47,216,108,21,154,199,130,125,149,194,224,25,];
   
   for i in 0..48 {
        assignment.x[0][0][i] = M31::from(x0_b0_a0_bytes[i]);
        assignment.x[0][1][i] = M31::from(x0_b0_a1_bytes[i]);
        assignment.x[1][0][i] = M31::from(x0_b1_a0_bytes[i]);
        assignment.x[1][1][i] = M31::from(x0_b1_a1_bytes[i]);
        assignment.x[2][0][i] = M31::from(x0_b2_a0_bytes[i]);
        assignment.x[2][1][i] = M31::from(x0_b2_a1_bytes[i]);
        assignment.y[0][0][i] = M31::from(x1_b0_a0_bytes[i]);
        assignment.y[0][1][i] = M31::from(x1_b0_a1_bytes[i]);
        assignment.y[1][0][i] = M31::from(x1_b1_a0_bytes[i]);
        assignment.y[1][1][i] = M31::from(x1_b1_a1_bytes[i]);
        assignment.y[2][0][i] = M31::from(x1_b2_a0_bytes[i]);
        assignment.y[2][1][i] = M31::from(x1_b2_a1_bytes[i]);
        assignment.z[0][0][i] = M31::from(x2_b0_a0_bytes[i]);
        assignment.z[0][1][i] = M31::from(x2_b0_a1_bytes[i]);
        assignment.z[1][0][i] = M31::from(x2_b1_a0_bytes[i]);
        assignment.z[1][1][i] = M31::from(x2_b1_a1_bytes[i]);
        assignment.z[2][0][i] = M31::from(x2_b2_a0_bytes[i]);
        assignment.z[2][1][i] = M31::from(x2_b2_a1_bytes[i]);
    }
    
    debug_eval(
        &E6AddCircuit::default(),
        &assignment,
        hint_registry,
    );
}

declare_circuit!(E6SubCircuit {
    x: [[[Variable; 48]; 2]; 3],
    y: [[[Variable; 48]; 2]; 3],
    z: [[[Variable; 48]; 2]; 3],
});

impl GenericDefine<M31Config> for E6SubCircuit<Variable> {
    fn define<Builder: RootAPI<M31Config>>(&self, builder: &mut Builder) {
        let mut ext6 = Ext6::new(builder);

        let x_e6 = GE6 {
            b0: GE2 {
                a0: new_internal_element(self.x[0][0].to_vec(), 0),
                a1: new_internal_element(self.x[0][1].to_vec(), 0),
            },
            b1: GE2 {
                a0: new_internal_element(self.x[1][0].to_vec(), 0),
                a1: new_internal_element(self.x[1][1].to_vec(), 0),
            },
            b2: GE2 {
                a0: new_internal_element(self.x[2][0].to_vec(), 0),
                a1: new_internal_element(self.x[2][1].to_vec(), 0),
            },
        };

        let y_e6 = GE6 {
            b0: GE2 {
                a0: new_internal_element(self.y[0][0].to_vec(), 0),
                a1: new_internal_element(self.y[0][1].to_vec(), 0),
            },
            b1: GE2 {
                a0: new_internal_element(self.y[1][0].to_vec(), 0),
                a1: new_internal_element(self.y[1][1].to_vec(), 0),
            },
            b2: GE2 {
                a0: new_internal_element(self.y[2][0].to_vec(), 0),
                a1: new_internal_element(self.y[2][1].to_vec(), 0),
            },
        };

        let z_e6 = GE6 {
            b0: GE2 {
                a0: new_internal_element(self.z[0][0].to_vec(), 0),
                a1: new_internal_element(self.z[0][1].to_vec(), 0),
            },
            b1: GE2 {
                a0: new_internal_element(self.z[1][0].to_vec(), 0),
                a1: new_internal_element(self.z[1][1].to_vec(), 0),
            },
            b2: GE2 {
                a0: new_internal_element(self.z[2][0].to_vec(), 0),
                a1: new_internal_element(self.z[2][1].to_vec(), 0),
            },
        };

        let mut z = ext6.sub(builder, &x_e6, &y_e6);

        ext6.assert_isequal(builder, &z, &z_e6);

        ext6.ext2.fp.check_mul(builder);
        ext6.ext2.fp.table.final_check(builder);
    }
}

#[test]
fn test_e6_sub() {
    let compile_result =
        compile_generic(&E6SubCircuit::default(), CompileOptions::default()).unwrap();
    let mut hint_registry = HintRegistry::<M31>::new();
    register_hint(&mut hint_registry);

    let mut assignment = E6SubCircuit::<M31> {
        x: [[[M31::from(0); 48]; 2]; 3],
        y: [[[M31::from(0); 48]; 2]; 3],
        z: [[[M31::from(0); 48]; 2]; 3],
    };

    let x0_b0_a0_bytes = [117,67,202,118,173,110,225,14,221,151,124,122,61,149,241,18,203,205,177,75,70,107,95,134,44,31,134,223,223,119,166,241,140,160,77,31,209,113,203,150,180,66,197,237,193,121,208,0,];
    let x0_b0_a1_bytes = [110,149,76,85,199,140,8,167,128,140,218,6,61,135,234,132,175,254,240,100,114,91,133,61,241,86,124,142,78,33,16,246,74,52,117,19,196,33,175,78,43,217,62,140,22,40,10,4,];
    let x0_b1_a0_bytes = [167,132,89,16,118,145,244,205,24,211,7,12,137,89,178,181,153,189,41,159,221,184,32,188,221,84,166,48,42,197,3,73,145,51,1,61,75,2,126,160,130,90,183,50,169,255,244,24,];
    let x0_b1_a1_bytes = [39,39,199,18,85,70,114,161,125,13,253,192,41,206,162,138,196,35,243,215,93,0,63,90,210,114,174,223,9,211,206,184,5,176,16,169,163,132,213,168,237,89,183,208,107,228,88,12,];
    let x0_b2_a0_bytes = [3,237,190,146,70,78,10,88,226,63,22,92,151,39,13,220,63,81,10,156,43,201,81,202,56,56,158,192,4,42,104,209,22,195,72,183,191,39,42,147,4,148,232,13,145,17,54,5,];
    let x0_b2_a1_bytes = [225,201,6,16,49,244,117,191,166,244,42,86,39,183,237,161,17,110,212,223,85,115,32,210,129,151,83,12,9,192,33,159,224,159,53,119,240,95,45,169,13,178,183,132,43,223,8,15,];
    let x1_b0_a0_bytes = [143,28,221,28,84,196,131,92,212,0,200,243,196,73,255,59,7,52,5,52,7,221,107,182,61,65,255,95,11,146,158,222,57,139,232,252,181,149,181,61,71,64,160,147,89,79,87,3,];
    let x1_b0_a1_bytes = [192,234,124,255,103,182,125,220,156,88,109,214,103,250,217,101,68,101,36,254,247,79,161,60,204,171,112,23,167,16,103,254,102,55,211,111,96,222,146,96,106,97,77,204,16,225,246,18,];
    let x1_b1_a0_bytes = [28,10,69,145,40,112,221,180,163,241,233,95,178,55,10,21,76,41,31,233,7,242,254,187,102,68,8,118,125,34,138,22,160,179,58,176,187,214,3,245,114,136,0,180,234,133,85,14,];
    let x1_b1_a1_bytes = [119,92,66,14,39,115,82,109,0,155,226,84,212,158,188,52,234,232,165,207,90,156,117,52,127,224,21,27,202,135,43,189,157,13,137,2,248,24,5,250,183,70,125,194,206,183,148,19,];
    let x1_b2_a0_bytes = [172,52,244,121,0,171,124,120,72,244,219,141,30,203,101,43,76,75,35,11,38,13,228,90,204,27,44,108,122,94,152,135,222,164,120,85,235,64,4,44,242,82,68,209,105,31,133,16,];
    let x1_b2_a1_bytes = [3,242,58,112,155,25,152,168,242,27,59,163,47,158,43,229,19,111,181,191,83,236,195,148,203,169,66,113,114,122,78,15,220,32,103,124,248,65,17,148,68,127,27,54,166,19,190,0,];
    let x2_b0_a0_bytes = [145,209,236,89,89,170,92,108,8,151,8,56,119,75,158,245,231,143,93,14,224,96,36,55,174,240,11,115,89,49,127,119,42,194,176,101,209,131,49,164,7,233,164,147,82,60,122,23,];
    let x2_b0_a1_bytes = [89,85,207,85,95,214,137,132,227,51,193,225,211,140,188,61,143,143,125,93,27,222,20,104,228,189,144,106,44,92,32,92,187,169,237,230,25,235,55,57,91,94,113,249,239,88,20,11,];
    let x2_b1_a0_bytes = [139,122,20,127,77,33,23,25,117,225,29,172,214,33,168,160,77,148,10,182,213,198,33,0,119,16,158,186,172,162,121,50,241,127,198,140,143,43,122,171,15,210,182,126,190,121,159,10,];
    let x2_b1_a1_bytes = [91,117,132,4,46,211,30,238,124,114,110,29,84,47,146,116,254,48,254,254,163,54,250,140,18,165,29,184,196,150,26,96,63,79,211,233,97,19,236,249,207,249,185,71,135,62,197,18,];
    let x2_b2_a0_bytes = [2,99,202,24,70,163,140,153,153,75,142,127,119,92,83,207,23,252,151,135,166,142,158,214,43,47,247,71,15,23,71,174,15,203,27,165,138,142,65,178,172,39,36,118,17,4,178,14,];
    let x2_b2_a1_bytes = [222,215,203,159,149,218,221,22,180,216,239,178,247,24,194,188,253,254,30,32,2,135,92,61,182,237,16,155,150,69,211,143,4,127,206,250,247,29,28,21,201,50,156,78,133,203,74,14,];

    for i in 0..48 {
        assignment.x[0][0][i] = M31::from(x0_b0_a0_bytes[i]);
        assignment.x[0][1][i] = M31::from(x0_b0_a1_bytes[i]);
        assignment.x[1][0][i] = M31::from(x0_b1_a0_bytes[i]);
        assignment.x[1][1][i] = M31::from(x0_b1_a1_bytes[i]);
        assignment.x[2][0][i] = M31::from(x0_b2_a0_bytes[i]);
        assignment.x[2][1][i] = M31::from(x0_b2_a1_bytes[i]);
        assignment.y[0][0][i] = M31::from(x1_b0_a0_bytes[i]);
        assignment.y[0][1][i] = M31::from(x1_b0_a1_bytes[i]);
        assignment.y[1][0][i] = M31::from(x1_b1_a0_bytes[i]);
        assignment.y[1][1][i] = M31::from(x1_b1_a1_bytes[i]);
        assignment.y[2][0][i] = M31::from(x1_b2_a0_bytes[i]);
        assignment.y[2][1][i] = M31::from(x1_b2_a1_bytes[i]);
        assignment.z[0][0][i] = M31::from(x2_b0_a0_bytes[i]);
        assignment.z[0][1][i] = M31::from(x2_b0_a1_bytes[i]);
        assignment.z[1][0][i] = M31::from(x2_b1_a0_bytes[i]);
        assignment.z[1][1][i] = M31::from(x2_b1_a1_bytes[i]);
        assignment.z[2][0][i] = M31::from(x2_b2_a0_bytes[i]);
        assignment.z[2][1][i] = M31::from(x2_b2_a1_bytes[i]);
    }
    
    debug_eval(
        &E6SubCircuit::default(),
        &assignment,
        hint_registry,
    );
}
declare_circuit!(E6MulCircuit {
    x: [[[Variable; 48]; 2]; 3],
    y: [[[Variable; 48]; 2]; 3],
    z: [[[Variable; 48]; 2]; 3],
});

impl GenericDefine<M31Config> for E6MulCircuit<Variable> {
    fn define<Builder: RootAPI<M31Config>>(&self, builder: &mut Builder) {
        let mut ext6 = Ext6::new(builder);

        let x_e6 = GE6 {
            b0: GE2 {
                a0: new_internal_element(self.x[0][0].to_vec(), 0),
                a1: new_internal_element(self.x[0][1].to_vec(), 0),
            },
            b1: GE2 {
                a0: new_internal_element(self.x[1][0].to_vec(), 0),
                a1: new_internal_element(self.x[1][1].to_vec(), 0),
            },
            b2: GE2 {
                a0: new_internal_element(self.x[2][0].to_vec(), 0),
                a1: new_internal_element(self.x[2][1].to_vec(), 0),
            },
        };

        let y_e6 = GE6 {
            b0: GE2 {
                a0: new_internal_element(self.y[0][0].to_vec(), 0),
                a1: new_internal_element(self.y[0][1].to_vec(), 0),
            },
            b1: GE2 {
                a0: new_internal_element(self.y[1][0].to_vec(), 0),
                a1: new_internal_element(self.y[1][1].to_vec(), 0),
            },
            b2: GE2 {
                a0: new_internal_element(self.y[2][0].to_vec(), 0),
                a1: new_internal_element(self.y[2][1].to_vec(), 0),
            },
        };

        let z_e6 = GE6 {
            b0: GE2 {
                a0: new_internal_element(self.z[0][0].to_vec(), 0),
                a1: new_internal_element(self.z[0][1].to_vec(), 0),
            },
            b1: GE2 {
                a0: new_internal_element(self.z[1][0].to_vec(), 0),
                a1: new_internal_element(self.z[1][1].to_vec(), 0),
            },
            b2: GE2 {
                a0: new_internal_element(self.z[2][0].to_vec(), 0),
                a1: new_internal_element(self.z[2][1].to_vec(), 0),
            },
        };

        let z = ext6.mul(builder, &x_e6, &y_e6);

        ext6.assert_isequal(builder, &z, &z_e6);

        ext6.ext2.fp.check_mul(builder);
        ext6.ext2.fp.table.final_check(builder);
    }
}

#[test]
fn test_e6_mul() {
    let compile_result =
        compile_generic(&E6MulCircuit::default(), CompileOptions::default()).unwrap();
    let mut hint_registry = HintRegistry::<M31>::new();
    register_hint(&mut hint_registry);

    let mut assignment = E6MulCircuit::<M31> {
        x: [[[M31::from(0); 48]; 2]; 3],
        y: [[[M31::from(0); 48]; 2]; 3],
        z: [[[M31::from(0); 48]; 2]; 3],
    };

    let x0_b0_a0_bytes = [46,171,188,186,190,115,108,16,106,47,30,48,92,33,24,187,243,219,27,71,225,210,31,244,228,11,110,205,138,94,101,51,32,146,68,158,91,248,87,49,113,45,18,9,66,223,1,9,];
    let x0_b0_a1_bytes = [200,59,2,153,8,53,214,186,105,82,243,109,164,109,113,140,250,42,7,118,205,121,7,142,25,196,1,120,181,155,93,59,47,9,39,56,222,243,229,81,42,190,234,135,29,21,58,10,];
    let x0_b1_a0_bytes = [44,28,34,122,59,250,97,234,89,159,141,225,198,102,238,93,2,213,43,132,40,208,140,196,58,226,107,20,163,33,14,18,176,3,23,16,30,125,126,32,22,190,71,210,30,191,219,11,];
    let x0_b1_a1_bytes = [117,245,238,225,186,36,41,224,112,118,52,177,6,63,94,95,195,156,135,55,66,238,102,19,236,170,247,0,192,35,113,135,126,252,180,6,19,225,9,182,205,4,15,215,223,141,27,12,];
    let x0_b2_a0_bytes = [39,225,139,50,21,53,177,230,184,63,137,162,135,228,11,252,62,38,15,226,82,118,68,100,144,193,13,144,106,160,183,126,103,164,151,4,93,223,90,137,128,105,212,176,142,231,9,13,];
    let x0_b2_a1_bytes = [13,33,87,166,233,45,135,152,194,168,223,42,131,60,4,47,58,198,193,106,193,188,61,167,198,143,154,46,53,12,174,127,82,235,72,155,54,216,81,166,76,250,194,201,20,170,145,14,];
    let x1_b0_a0_bytes = [2,211,218,184,13,175,37,119,109,40,212,219,183,74,233,163,185,243,126,237,106,186,211,233,160,102,0,230,100,165,248,28,96,119,174,107,209,142,190,193,152,62,155,175,169,70,198,1,];
    let x1_b0_a1_bytes = [2,133,167,173,76,108,164,230,130,110,187,191,213,215,105,214,206,183,176,90,84,70,109,18,236,29,96,101,149,41,37,218,71,92,40,234,134,231,239,125,255,90,112,176,182,248,118,3,];
    let x1_b1_a0_bytes = [84,102,133,136,37,82,182,154,143,152,228,7,202,193,77,174,99,19,163,168,144,32,47,97,46,107,52,174,168,67,202,93,144,247,196,217,179,40,147,112,208,95,228,191,236,175,23,21,];
    let x1_b1_a1_bytes = [250,209,134,38,35,182,176,144,176,100,39,18,144,67,229,122,63,26,6,185,14,76,77,69,198,73,252,148,179,201,15,229,74,147,206,37,103,84,160,82,223,173,206,135,34,221,149,19,];
    let x1_b2_a0_bytes = [78,219,161,76,22,59,94,124,156,131,175,147,51,145,148,54,54,193,166,92,244,72,183,189,189,119,33,102,90,90,228,193,246,103,108,63,181,50,240,142,75,148,11,253,219,175,4,18,];
    let x1_b2_a1_bytes = [157,255,244,149,96,149,68,19,16,227,89,166,192,157,80,183,121,211,186,8,244,156,202,65,14,189,252,38,110,38,172,34,136,186,155,102,39,200,132,159,155,58,186,36,41,164,111,20,];
    let x2_b0_a0_bytes = [139,57,43,3,203,41,159,16,165,223,135,253,137,144,225,68,65,203,47,32,3,82,64,122,20,104,160,155,106,139,224,96,40,95,114,1,213,182,187,111,179,56,224,4,45,79,115,19,];
    let x2_b0_a1_bytes = [182,46,28,46,128,147,103,169,72,64,229,0,37,163,104,210,193,180,172,228,228,129,16,194,11,41,55,53,204,163,74,69,245,7,24,42,79,15,171,228,122,254,81,177,236,102,202,9,];
    let x2_b1_a0_bytes = [198,127,46,145,88,18,205,163,244,216,212,57,7,225,227,66,178,27,48,206,191,120,8,212,167,146,38,34,123,43,223,50,131,109,49,118,100,5,30,194,25,89,176,3,231,181,38,18,];
    let x2_b1_a1_bytes = [194,218,15,76,86,206,59,118,75,9,124,137,170,6,84,184,125,247,228,139,152,171,125,242,137,199,170,11,116,83,40,184,189,14,93,195,111,138,213,242,212,90,128,60,50,132,69,0,];
    let x2_b2_a0_bytes = [239,2,119,9,143,45,156,90,96,201,15,104,44,158,202,13,109,55,21,111,75,182,173,240,31,203,253,85,116,120,118,81,170,84,219,136,90,225,140,106,110,222,193,62,128,47,233,3,];
    let x2_b2_a1_bytes = [163,224,214,44,217,30,86,63,64,74,49,222,85,74,144,121,178,207,115,64,58,69,243,3,42,210,225,158,53,32,60,206,224,25,208,203,198,36,195,177,49,37,9,229,194,16,66,13,];

    for i in 0..48 {
        assignment.x[0][0][i] = M31::from(x0_b0_a0_bytes[i]);
        assignment.x[0][1][i] = M31::from(x0_b0_a1_bytes[i]);
        assignment.x[1][0][i] = M31::from(x0_b1_a0_bytes[i]);
        assignment.x[1][1][i] = M31::from(x0_b1_a1_bytes[i]);
        assignment.x[2][0][i] = M31::from(x0_b2_a0_bytes[i]);
        assignment.x[2][1][i] = M31::from(x0_b2_a1_bytes[i]);
        assignment.y[0][0][i] = M31::from(x1_b0_a0_bytes[i]);
        assignment.y[0][1][i] = M31::from(x1_b0_a1_bytes[i]);
        assignment.y[1][0][i] = M31::from(x1_b1_a0_bytes[i]);
        assignment.y[1][1][i] = M31::from(x1_b1_a1_bytes[i]);
        assignment.y[2][0][i] = M31::from(x1_b2_a0_bytes[i]);
        assignment.y[2][1][i] = M31::from(x1_b2_a1_bytes[i]);
        assignment.z[0][0][i] = M31::from(x2_b0_a0_bytes[i]);
        assignment.z[0][1][i] = M31::from(x2_b0_a1_bytes[i]);
        assignment.z[1][0][i] = M31::from(x2_b1_a0_bytes[i]);
        assignment.z[1][1][i] = M31::from(x2_b1_a1_bytes[i]);
        assignment.z[2][0][i] = M31::from(x2_b2_a0_bytes[i]);
        assignment.z[2][1][i] = M31::from(x2_b2_a1_bytes[i]);
    }

    debug_eval(&E6MulCircuit::default(), &assignment, hint_registry);
}

declare_circuit!(E6SquareCircuit {
    x: [[[Variable; 48]; 2]; 3],
    z: [[[Variable; 48]; 2]; 3],
});

impl GenericDefine<M31Config> for E6SquareCircuit<Variable> {
    fn define<Builder: RootAPI<M31Config>>(&self, builder: &mut Builder) {
        let mut ext6 = Ext6::new(builder);

        let x_e6 = GE6 {
            b0: GE2 {
                a0: new_internal_element(self.x[0][0].to_vec(), 0),
                a1: new_internal_element(self.x[0][1].to_vec(), 0),
            },
            b1: GE2 {
                a0: new_internal_element(self.x[1][0].to_vec(), 0),
                a1: new_internal_element(self.x[1][1].to_vec(), 0),
            },
            b2: GE2 {
                a0: new_internal_element(self.x[2][0].to_vec(), 0),
                a1: new_internal_element(self.x[2][1].to_vec(), 0),
            },
        };

        let z_e6 = GE6 {
            b0: GE2 {
                a0: new_internal_element(self.z[0][0].to_vec(), 0),
                a1: new_internal_element(self.z[0][1].to_vec(), 0),
            },
            b1: GE2 {
                a0: new_internal_element(self.z[1][0].to_vec(), 0),
                a1: new_internal_element(self.z[1][1].to_vec(), 0),
            },
            b2: GE2 {
                a0: new_internal_element(self.z[2][0].to_vec(), 0),
                a1: new_internal_element(self.z[2][1].to_vec(), 0),
            },
        };

        let z = ext6.square(builder, &x_e6);

        ext6.assert_isequal(builder, &z, &z_e6);

        ext6.ext2.fp.check_mul(builder);
        ext6.ext2.fp.table.final_check(builder);
    }
}

#[test]
fn test_e6_square() {
    let compile_result =
        compile_generic(&E6SquareCircuit::default(), CompileOptions::default()).unwrap();
    let mut hint_registry = HintRegistry::<M31>::new();
    register_hint(&mut hint_registry);

    let mut assignment = E6SquareCircuit::<M31> {
        x: [[[M31::from(0); 48]; 2]; 3],
        z: [[[M31::from(0); 48]; 2]; 3],
    };

    let x0_b0_a0_bytes = [149,252,160,161,66,108,73,228,243,168,88,37,39,191,205,98,241,61,156,45,52,99,67,183,178,209,195,34,3,60,173,58,42,202,210,5,243,177,190,5,100,201,100,209,177,231,187,21,];
    let x0_b0_a1_bytes = [71,251,181,71,28,134,218,38,21,32,1,21,12,198,125,39,126,54,18,10,211,211,104,12,203,201,22,109,65,3,1,27,81,91,222,53,40,245,103,137,79,164,255,137,145,160,203,14,];
    let x0_b1_a0_bytes = [205,77,53,46,150,38,185,19,233,44,84,29,158,181,240,47,163,3,60,164,129,252,205,122,22,84,219,0,146,112,155,9,115,133,84,26,18,164,163,46,177,9,213,50,103,38,251,19,];
    let x0_b1_a1_bytes = [223,114,215,138,45,155,174,77,6,236,176,6,65,105,33,159,192,203,32,175,68,156,172,222,85,103,32,36,253,197,35,30,173,48,57,212,101,214,118,190,92,26,177,126,37,200,151,0,];
    let x0_b2_a0_bytes = [111,205,175,51,14,14,198,159,176,90,194,167,0,56,230,245,50,250,31,186,192,108,141,75,129,86,203,69,3,152,246,84,135,11,208,177,161,143,194,0,99,6,201,91,5,202,196,25,];
    let x0_b2_a1_bytes = [99,11,232,254,225,220,249,134,36,14,216,116,146,232,227,0,25,38,227,90,221,113,88,108,85,40,251,88,105,103,27,208,30,113,129,203,249,108,144,154,211,251,107,12,168,105,81,1,];
    let x2_b0_a0_bytes = [21,61,58,202,150,61,40,78,118,188,60,67,131,26,108,110,94,101,43,230,149,87,4,207,232,27,6,220,59,150,3,211,185,62,139,123,205,7,160,187,143,73,151,82,50,160,193,21,];
    let x2_b0_a1_bytes = [84,111,79,158,196,154,235,30,225,34,147,112,32,10,3,32,32,18,230,244,84,230,163,116,200,228,152,247,75,60,129,62,23,205,10,243,139,55,149,133,138,253,102,67,135,148,215,12,];
    let x2_b1_a0_bytes = [252,95,170,53,240,79,250,214,195,45,219,214,5,204,25,135,59,205,74,233,211,96,45,236,68,55,107,182,36,114,211,245,43,119,254,19,178,186,73,240,160,164,21,145,101,105,34,14,];
    let x2_b1_a1_bytes = [36,26,27,52,88,138,91,54,24,252,143,17,39,84,137,8,191,39,110,10,128,92,128,150,191,216,22,202,75,194,99,92,20,247,159,212,122,217,46,186,86,242,95,187,128,14,38,5,];
    let x2_b2_a0_bytes = [193,78,94,37,120,49,230,20,47,17,14,25,228,74,163,207,94,107,42,232,230,107,131,61,250,195,232,77,250,90,114,234,173,250,168,6,172,100,78,35,121,210,81,97,89,82,156,17,];
    let x2_b2_a1_bytes = [22,126,225,109,245,84,53,66,154,187,48,16,56,105,180,247,79,94,107,74,174,39,224,37,9,10,74,204,85,33,2,165,244,66,179,232,52,28,97,71,5,169,96,142,213,59,47,19,];

    
    for i in 0..48 {
        assignment.x[0][0][i] = M31::from(x0_b0_a0_bytes[i]);
        assignment.x[0][1][i] = M31::from(x0_b0_a1_bytes[i]);
        assignment.x[1][0][i] = M31::from(x0_b1_a0_bytes[i]);
        assignment.x[1][1][i] = M31::from(x0_b1_a1_bytes[i]);
        assignment.x[2][0][i] = M31::from(x0_b2_a0_bytes[i]);
        assignment.x[2][1][i] = M31::from(x0_b2_a1_bytes[i]);
        assignment.z[0][0][i] = M31::from(x2_b0_a0_bytes[i]);
        assignment.z[0][1][i] = M31::from(x2_b0_a1_bytes[i]);
        assignment.z[1][0][i] = M31::from(x2_b1_a0_bytes[i]);
        assignment.z[1][1][i] = M31::from(x2_b1_a1_bytes[i]);
        assignment.z[2][0][i] = M31::from(x2_b2_a0_bytes[i]);
        assignment.z[2][1][i] = M31::from(x2_b2_a1_bytes[i]);
    }

    debug_eval(&E6SquareCircuit::default(), &assignment, hint_registry);
}

declare_circuit!(E6DivCircuit {
    x: [[[Variable; 48]; 2]; 3],
    y: [[[Variable; 48]; 2]; 3],
    z: [[[Variable; 48]; 2]; 3],
});

impl GenericDefine<M31Config> for E6DivCircuit<Variable> {
    fn define<Builder: RootAPI<M31Config>>(&self, builder: &mut Builder) {
        let mut ext6 = Ext6::new(builder);

        let x_e6 = GE6 {
            b0: GE2 {
                a0: new_internal_element(self.x[0][0].to_vec(), 0),
                a1: new_internal_element(self.x[0][1].to_vec(), 0),
            },
            b1: GE2 {
                a0: new_internal_element(self.x[1][0].to_vec(), 0),
                a1: new_internal_element(self.x[1][1].to_vec(), 0),
            },
            b2: GE2 {
                a0: new_internal_element(self.x[2][0].to_vec(), 0),
                a1: new_internal_element(self.x[2][1].to_vec(), 0),
            },
        };

        let y_e6 = GE6 {
            b0: GE2 {
                a0: new_internal_element(self.y[0][0].to_vec(), 0),
                a1: new_internal_element(self.y[0][1].to_vec(), 0),
            },
            b1: GE2 {
                a0: new_internal_element(self.y[1][0].to_vec(), 0),
                a1: new_internal_element(self.y[1][1].to_vec(), 0),
            },
            b2: GE2 {
                a0: new_internal_element(self.y[2][0].to_vec(), 0),
                a1: new_internal_element(self.y[2][1].to_vec(), 0),
            },
        };

        let z_e6 = GE6 {
            b0: GE2 {
                a0: new_internal_element(self.z[0][0].to_vec(), 0),
                a1: new_internal_element(self.z[0][1].to_vec(), 0),
            },
            b1: GE2 {
                a0: new_internal_element(self.z[1][0].to_vec(), 0),
                a1: new_internal_element(self.z[1][1].to_vec(), 0),
            },
            b2: GE2 {
                a0: new_internal_element(self.z[2][0].to_vec(), 0),
                a1: new_internal_element(self.z[2][1].to_vec(), 0),
            },
        };

        let z = ext6.div(builder, &x_e6, &y_e6);

        ext6.assert_isequal(builder, &z, &z_e6);

        ext6.ext2.fp.check_mul(builder);
        ext6.ext2.fp.table.final_check(builder);
    }
}

#[test]
fn test_e6_div() {
    // let compile_result =
    //     compile_generic(&E6DivCircuit::default(), CompileOptions::default()).unwrap();
    let mut hint_registry = HintRegistry::<M31>::new();
    register_hint(&mut hint_registry);

    let mut assignment = E6DivCircuit::<M31> {
        x: [[[M31::from(0); 48]; 2]; 3],
        y: [[[M31::from(0); 48]; 2]; 3],
        z: [[[M31::from(0); 48]; 2]; 3]
    };

    let x0_b0_a0_bytes = [107,46,111,157,84,135,89,107,29,18,126,99,75,231,135,136,247,175,57,99,90,48,149,234,25,93,172,7,58,116,96,138,58,167,206,46,194,47,132,61,81,255,143,139,9,178,179,24,];
    let x0_b0_a1_bytes = [65,150,235,198,199,204,132,179,17,239,168,83,18,235,124,242,186,37,23,63,212,62,143,188,225,59,144,230,131,184,85,242,107,221,207,52,189,231,244,131,25,123,52,56,61,9,22,20,];
    let x0_b1_a0_bytes = [173,39,135,175,251,127,251,89,158,139,94,66,180,143,155,50,213,196,158,102,168,240,200,30,74,10,136,214,182,205,96,211,42,67,117,205,187,245,70,16,253,106,190,159,65,142,118,12,];
    let x0_b1_a1_bytes = [231,106,130,80,207,77,88,201,127,90,167,140,61,4,133,64,239,153,233,31,153,238,25,23,203,39,59,37,7,191,226,200,133,35,91,114,57,124,77,70,252,40,241,60,103,188,249,23,];
    let x0_b2_a0_bytes = [119,50,63,185,207,181,225,181,10,24,209,197,165,151,189,133,107,135,22,230,46,166,178,27,159,132,48,130,126,52,108,36,236,227,27,98,88,15,205,18,147,23,65,177,186,202,219,19,];
    let x0_b2_a1_bytes = [165,58,17,37,247,187,48,54,42,252,33,95,119,174,86,195,0,104,57,143,164,118,207,61,240,19,145,50,187,85,46,215,93,133,181,13,96,65,146,185,132,116,84,145,253,103,193,19,];
    let x1_b0_a0_bytes = [16,79,32,49,174,6,172,207,122,139,231,68,149,199,95,98,12,84,238,96,101,210,104,62,64,216,27,120,43,210,103,245,8,199,91,75,67,163,246,235,19,66,153,185,41,186,103,5,];
    let x1_b0_a1_bytes = [57,238,57,195,235,52,131,101,220,163,24,39,229,83,27,121,219,17,39,82,86,239,237,251,127,220,229,92,111,31,58,175,86,76,37,169,23,148,115,146,124,241,174,228,149,9,90,6,];
    let x1_b1_a0_bytes = [247,148,68,210,199,239,86,29,204,205,220,164,22,11,24,35,228,244,237,116,25,70,189,251,247,70,117,156,224,249,17,138,63,50,78,4,155,91,30,26,123,159,172,23,130,144,43,25,];
    let x1_b1_a1_bytes = [60,103,177,115,150,175,97,91,229,107,241,226,110,3,139,96,108,37,224,144,45,117,18,230,93,140,255,15,131,111,155,73,142,169,96,196,69,110,227,144,70,184,233,207,145,70,3,0,];
    let x1_b2_a0_bytes = [199,33,152,245,103,119,131,68,162,115,65,191,82,228,118,227,249,183,102,194,217,231,28,41,83,99,36,244,250,58,231,247,65,63,127,246,254,218,128,63,150,53,205,127,25,160,45,21,];
    let x1_b2_a1_bytes = [149,118,225,27,180,204,98,78,29,25,184,252,36,166,66,106,123,142,80,56,225,137,128,130,194,102,142,115,42,12,187,161,9,23,9,34,199,12,73,213,22,80,114,193,138,69,67,16,];
    let x2_b0_a0_bytes = [90,197,146,236,129,61,116,59,100,18,45,130,188,202,114,151,175,48,14,125,137,143,100,130,199,246,11,98,206,173,27,90,238,217,195,190,244,184,44,110,36,35,90,250,84,187,120,11,];
    let x2_b0_a1_bytes = [156,140,120,55,221,129,220,124,199,65,79,230,109,209,226,177,66,182,240,70,63,51,79,248,163,108,109,49,94,187,20,174,22,226,131,36,33,33,148,76,96,169,72,146,78,134,169,22,];
    let x2_b1_a0_bytes = [164,204,252,143,75,2,19,248,173,72,189,106,203,49,221,71,109,218,238,90,49,209,82,251,197,96,219,145,69,188,129,219,65,76,185,220,97,253,231,125,226,252,178,159,83,25,55,13,];
    let x2_b1_a1_bytes = [191,109,242,246,21,112,126,212,129,232,137,91,89,38,9,142,25,97,38,146,30,113,12,214,44,194,123,45,28,142,124,137,153,160,18,38,250,208,129,46,181,60,20,233,105,102,124,12,];
    let x2_b2_a0_bytes = [222,43,171,59,32,102,33,247,125,121,241,64,19,99,21,169,182,203,33,160,245,2,234,186,2,46,154,173,209,58,169,112,207,46,35,152,250,162,239,99,154,73,56,209,26,4,113,21,];
    let x2_b2_a1_bytes = [228,214,111,241,243,60,177,143,184,255,55,230,82,186,163,92,237,57,148,219,0,129,130,243,246,252,253,72,173,70,236,178,95,186,219,127,127,214,36,192,161,233,161,237,197,138,146,16,];
    
    for i in 0..48 {
        assignment.x[0][0][i] = M31::from(x0_b0_a0_bytes[i]);
        assignment.x[0][1][i] = M31::from(x0_b0_a1_bytes[i]);
        assignment.x[1][0][i] = M31::from(x0_b1_a0_bytes[i]);
        assignment.x[1][1][i] = M31::from(x0_b1_a1_bytes[i]);
        assignment.x[2][0][i] = M31::from(x0_b2_a0_bytes[i]);
        assignment.x[2][1][i] = M31::from(x0_b2_a1_bytes[i]);
        assignment.y[0][0][i] = M31::from(x1_b0_a0_bytes[i]);
        assignment.y[0][1][i] = M31::from(x1_b0_a1_bytes[i]);
        assignment.y[1][0][i] = M31::from(x1_b1_a0_bytes[i]);
        assignment.y[1][1][i] = M31::from(x1_b1_a1_bytes[i]);
        assignment.y[2][0][i] = M31::from(x1_b2_a0_bytes[i]);
        assignment.y[2][1][i] = M31::from(x1_b2_a1_bytes[i]);
        assignment.z[0][0][i] = M31::from(x2_b0_a0_bytes[i]);
        assignment.z[0][1][i] = M31::from(x2_b0_a1_bytes[i]);
        assignment.z[1][0][i] = M31::from(x2_b1_a0_bytes[i]);
        assignment.z[1][1][i] = M31::from(x2_b1_a1_bytes[i]);
        assignment.z[2][0][i] = M31::from(x2_b2_a0_bytes[i]);
        assignment.z[2][1][i] = M31::from(x2_b2_a1_bytes[i]);
    }

    debug_eval(&E6DivCircuit::default(), &assignment, hint_registry);
}   

declare_circuit!(E6MulByNonResidueCircuit {
    a: [[[Variable; 48]; 2]; 3],
    c: [[[Variable; 48]; 2]; 3], // Public variable
});

impl GenericDefine<M31Config> for E6MulByNonResidueCircuit<Variable> {
    fn define<Builder: RootAPI<M31Config>>(&self, builder: &mut Builder) {
        let mut ext6 = Ext6::new(builder);

        let a_e6 = GE6 {
            b0: GE2 {
                a0: new_internal_element(self.a[0][0].to_vec(), 0),
                a1: new_internal_element(self.a[0][1].to_vec(), 0),
            },
            b1: GE2 {
                a0: new_internal_element(self.a[1][0].to_vec(), 0),
                a1: new_internal_element(self.a[1][1].to_vec(), 0),
            },
            b2: GE2 {
                a0: new_internal_element(self.a[2][0].to_vec(), 0),
                a1: new_internal_element(self.a[2][1].to_vec(), 0),
            },
        };

        let c_e6 = GE6 {
            b0: GE2 {
                a0: new_internal_element(self.c[0][0].to_vec(), 0),
                a1: new_internal_element(self.c[0][1].to_vec(), 0),
            },
            b1: GE2 {
                a0: new_internal_element(self.c[1][0].to_vec(), 0),
                a1: new_internal_element(self.c[1][1].to_vec(), 0),
            },
            b2: GE2 {
                a0: new_internal_element(self.c[2][0].to_vec(), 0),
                a1: new_internal_element(self.c[2][1].to_vec(), 0),
            },
        };

        let result = ext6.mul_by_non_residue(builder, &a_e6);

        ext6.assert_isequal(builder, &result, &c_e6);

        ext6.ext2.fp.check_mul(builder);
        ext6.ext2.fp.table.final_check(builder);
    }
}

#[test]
fn test_e6_mul_by_non_residue() {
    let compile_result =
        compile_generic(&E6MulByNonResidueCircuit::default(), CompileOptions::default()).unwrap();
    let mut hint_registry = HintRegistry::<M31>::new();
    register_hint(&mut hint_registry); // Updated hint registration

    let mut assignment = E6MulByNonResidueCircuit::<M31> {
        a: [[[M31::from(0); 48]; 2]; 3],
        c: [[[M31::from(0); 48]; 2]; 3],
    };

    let x0_b0_a0_bytes = [64,88,27,110,238,39,175,216,0,29,131,126,214,115,176,254,76,55,0,215,59,70,40,219,237,215,146,219,178,177,230,83,93,215,207,32,189,190,197,133,30,113,224,95,33,111,88,0,];
    let x0_b0_a1_bytes = [122,78,181,224,62,88,174,158,82,231,130,108,51,204,90,167,55,38,234,69,242,182,217,230,63,135,52,193,222,71,109,97,201,228,118,32,66,97,177,39,136,245,14,185,224,252,41,16,];
    let x0_b1_a0_bytes = [19,197,36,21,31,161,152,225,90,247,154,217,54,210,113,218,37,48,18,232,196,128,209,136,220,3,88,71,54,180,158,44,100,135,14,96,125,46,82,140,201,53,79,149,38,100,5,6,];
    let x0_b1_a1_bytes = [59,16,99,177,130,33,110,86,138,187,1,227,142,131,36,234,164,215,71,206,79,145,201,34,138,244,1,46,141,35,110,92,237,207,216,108,22,224,70,148,146,55,87,189,20,82,12,17,];
    let x0_b2_a0_bytes = [74,190,238,44,234,56,156,176,254,232,115,121,131,101,133,143,203,79,126,36,45,89,244,171,139,36,88,144,76,160,27,232,239,54,71,229,147,4,218,192,199,157,95,79,10,1,249,11,];
    let x0_b2_a1_bytes = [180,248,244,93,213,144,28,114,150,60,209,143,249,0,232,139,255,201,20,252,109,69,225,215,17,242,137,229,0,49,158,32,234,225,207,223,55,93,15,83,134,142,58,203,248,80,179,11,];
    let x2_b0_a0_bytes = [150,197,249,206,20,168,127,62,104,172,162,233,137,100,157,3,204,133,105,40,191,19,19,212,121,50,206,170,75,111,125,199,5,85,119,5,92,167,202,109,65,15,37,132,17,176,69,0,];
    let x2_b0_a1_bytes = [254,182,227,138,191,201,184,34,149,37,69,9,125,102,109,27,203,25,147,32,155,158,213,131,157,22,226,117,77,209,185,8,218,24,23,197,203,97,233,19,78,44,154,26,3,82,172,23,];
    let x2_b1_a0_bytes = [64,88,27,110,238,39,175,216,0,29,131,126,214,115,176,254,76,55,0,215,59,70,40,219,237,215,146,219,178,177,230,83,93,215,207,32,189,190,197,133,30,113,224,95,33,111,88,0,];
    let x2_b1_a1_bytes = [122,78,181,224,62,88,174,158,82,231,130,108,51,204,90,167,55,38,234,69,242,182,217,230,63,135,52,193,222,71,109,97,201,228,118,32,66,97,177,39,136,245,14,185,224,252,41,16,];
    let x2_b2_a0_bytes = [19,197,36,21,31,161,152,225,90,247,154,217,54,210,113,218,37,48,18,232,196,128,209,136,220,3,88,71,54,180,158,44,100,135,14,96,125,46,82,140,201,53,79,149,38,100,5,6,];
    let x2_b2_a1_bytes = [59,16,99,177,130,33,110,86,138,187,1,227,142,131,36,234,164,215,71,206,79,145,201,34,138,244,1,46,141,35,110,92,237,207,216,108,22,224,70,148,146,55,87,189,20,82,12,17,];

    for i in 0..48 {
        assignment.a[0][0][i] = M31::from(x0_b0_a0_bytes[i]);
        assignment.a[0][1][i] = M31::from(x0_b0_a1_bytes[i]);
        assignment.a[1][0][i] = M31::from(x0_b1_a0_bytes[i]);
        assignment.a[1][1][i] = M31::from(x0_b1_a1_bytes[i]);
        assignment.a[2][0][i] = M31::from(x0_b2_a0_bytes[i]);
        assignment.a[2][1][i] = M31::from(x0_b2_a1_bytes[i]);
        assignment.c[0][0][i] = M31::from(x2_b0_a0_bytes[i]);
        assignment.c[0][1][i] = M31::from(x2_b0_a1_bytes[i]);
        assignment.c[1][0][i] = M31::from(x2_b1_a0_bytes[i]);
        assignment.c[1][1][i] = M31::from(x2_b1_a1_bytes[i]);
        assignment.c[2][0][i] = M31::from(x2_b2_a0_bytes[i]);
        assignment.c[2][1][i] = M31::from(x2_b2_a1_bytes[i]);
    }

    debug_eval(&E6MulByNonResidueCircuit::default(), &assignment, hint_registry);
}
declare_circuit!(E6MulByE2Circuit {
    a: [[[Variable; 48]; 2]; 3],
    b: [[Variable; 48]; 2],
    c: [[[Variable; 48]; 2]; 3], // Public variable
});

impl GenericDefine<M31Config> for E6MulByE2Circuit<Variable> {
    fn define<Builder: RootAPI<M31Config>>(&self, builder: &mut Builder) {
        let mut ext6 = Ext6::new(builder);

        let a_e6 = GE6 {
            b0: GE2 {
                a0: new_internal_element(self.a[0][0].to_vec(), 0),
                a1: new_internal_element(self.a[0][1].to_vec(), 0),
            },
            b1: GE2 {
                a0: new_internal_element(self.a[1][0].to_vec(), 0),
                a1: new_internal_element(self.a[1][1].to_vec(), 0),
            },
            b2: GE2 {
                a0: new_internal_element(self.a[2][0].to_vec(), 0),
                a1: new_internal_element(self.a[2][1].to_vec(), 0),
            },
        };

        let b_e2 = GE2 {
            a0: new_internal_element(self.b[0].to_vec(), 0),
            a1: new_internal_element(self.b[1].to_vec(), 0),
        };

        let c_e6 = GE6 {
            b0: GE2 {
                a0: new_internal_element(self.c[0][0].to_vec(), 0),
                a1: new_internal_element(self.c[0][1].to_vec(), 0),
            },
            b1: GE2 {
                a0: new_internal_element(self.c[1][0].to_vec(), 0),
                a1: new_internal_element(self.c[1][1].to_vec(), 0),
            },
            b2: GE2 {
                a0: new_internal_element(self.c[2][0].to_vec(), 0),
                a1: new_internal_element(self.c[2][1].to_vec(), 0),
            },
        };

        let result = ext6.mul_by_e2(builder, &a_e6, &b_e2);

        ext6.assert_isequal(builder, &result, &c_e6);

        ext6.ext2.fp.check_mul(builder);
        ext6.ext2.fp.table.final_check(builder);
    }
}

#[test]
fn test_e6_mul_by_e2() {
    let compile_result =
        compile_generic(&E6MulByE2Circuit::default(), CompileOptions::default()).unwrap();
    let mut hint_registry = HintRegistry::<M31>::new();
    register_hint(&mut hint_registry);

    let mut assignment = E6MulByE2Circuit::<M31> {
        a: [[[M31::from(0); 48]; 2]; 3],
        b: [[M31::from(0); 48]; 2],
        c: [[[M31::from(0); 48]; 2]; 3],
    };

    let x0_b0_a0_bytes = [16,57,17,157,215,105,216,201,10,247,112,166,181,199,152,28,187,8,152,145,14,226,75,178,88,143,56,117,1,55,178,123,152,85,192,63,120,146,235,227,59,102,139,161,232,201,13,15,];
    let x0_b0_a1_bytes = [147,25,67,213,252,165,176,151,237,58,214,37,92,194,214,83,112,89,63,174,49,236,181,205,144,131,107,113,212,194,51,103,59,21,254,228,22,100,72,56,115,145,130,37,159,1,86,9,];
    let x0_b1_a0_bytes = [110,7,174,136,163,166,185,216,13,253,185,54,98,138,172,69,174,201,224,173,136,39,104,115,49,121,205,32,41,60,211,20,121,127,59,21,232,21,70,229,85,167,158,220,206,194,61,13,];
    let x0_b1_a1_bytes = [202,174,161,164,127,100,139,170,157,175,150,48,67,211,86,114,98,112,118,3,114,72,79,21,159,94,217,155,248,141,225,169,226,250,129,40,158,219,156,118,90,99,244,64,66,206,74,21,];
    let x0_b2_a0_bytes = [215,144,182,192,19,102,21,232,158,9,31,130,212,188,238,38,170,19,229,84,75,24,111,142,45,145,229,48,24,184,233,158,38,62,101,186,114,91,221,55,65,177,108,67,158,124,155,9,];
    let x0_b2_a1_bytes = [64,44,116,89,206,11,228,146,252,236,146,29,185,236,100,94,122,98,78,87,177,244,214,2,13,132,236,195,65,161,227,70,108,189,17,229,3,52,169,45,226,64,174,22,254,15,191,12,];
    let x1_a0_bytes = [114,106,253,79,101,99,40,6,197,30,178,73,223,122,42,247,149,236,253,200,209,115,97,199,100,27,124,167,186,36,238,0,217,9,223,217,47,188,242,234,223,225,128,69,157,221,219,12,];
    let x1_a1_bytes = [124,98,167,48,13,100,22,101,244,251,76,109,36,17,221,126,147,35,171,78,158,4,185,1,216,28,6,58,116,108,163,8,182,253,15,51,79,123,131,108,64,10,160,56,244,55,72,7,];
    let x2_b0_a0_bytes = [153,55,58,153,36,139,91,1,157,142,175,89,153,215,36,153,112,24,223,137,246,136,0,233,164,171,128,99,192,200,94,71,91,98,71,192,102,137,106,60,158,122,239,0,147,81,179,5,];
    let x2_b0_a1_bytes = [173,66,149,241,216,131,213,206,107,1,169,230,249,39,185,87,1,148,238,174,23,178,86,73,54,92,238,174,43,198,127,81,163,84,151,138,197,159,230,81,0,78,116,244,147,43,211,4,];
    let x2_b1_a0_bytes = [62,157,10,199,254,78,13,97,44,120,224,70,91,75,226,66,53,202,111,148,237,182,102,239,86,42,226,26,238,35,85,252,219,84,202,237,73,130,254,21,215,62,251,87,198,30,87,21,];
    let x2_b1_a1_bytes = [118,55,226,164,64,86,177,125,35,181,228,222,21,244,209,30,48,165,18,136,74,152,217,237,180,21,74,136,35,36,224,236,200,90,169,148,75,14,110,250,159,162,149,221,95,147,151,17,];
    let x2_b2_a0_bytes = [178,231,158,80,57,45,61,51,192,173,128,149,51,219,187,6,27,224,109,58,182,90,23,59,58,241,11,39,250,215,241,128,16,22,140,42,141,122,205,52,39,245,102,215,23,174,254,10,];
    let x2_b2_a1_bytes = [56,187,148,53,25,217,226,99,85,254,164,111,88,109,86,6,250,129,217,211,222,9,171,190,246,148,132,90,176,253,247,67,72,186,177,65,187,205,117,234,105,70,3,215,194,53,158,13,];

    for i in 0..48 {
        assignment.a[0][0][i] = M31::from(x0_b0_a0_bytes[i]);
        assignment.a[0][1][i] = M31::from(x0_b0_a1_bytes[i]);
        assignment.a[1][0][i] = M31::from(x0_b1_a0_bytes[i]);
        assignment.a[1][1][i] = M31::from(x0_b1_a1_bytes[i]);
        assignment.a[2][0][i] = M31::from(x0_b2_a0_bytes[i]);
        assignment.a[2][1][i] = M31::from(x0_b2_a1_bytes[i]);
        assignment.b[0][i] = M31::from(x1_a0_bytes[i]);
        assignment.b[1][i] = M31::from(x1_a1_bytes[i]);
        assignment.c[0][0][i] = M31::from(x2_b0_a0_bytes[i]);
        assignment.c[0][1][i] = M31::from(x2_b0_a1_bytes[i]);
        assignment.c[1][0][i] = M31::from(x2_b1_a0_bytes[i]);
        assignment.c[1][1][i] = M31::from(x2_b1_a1_bytes[i]);
        assignment.c[2][0][i] = M31::from(x2_b2_a0_bytes[i]);
        assignment.c[2][1][i] = M31::from(x2_b2_a1_bytes[i]);
    }

    debug_eval(&E6MulByE2Circuit::default(), &assignment, hint_registry);
}

declare_circuit!(E6MulBy01Circuit {
    a: [[[Variable; 48]; 2]; 3],
    c0: [[Variable; 48]; 2],
    c1: [[Variable; 48]; 2],
    c: [[[Variable; 48]; 2]; 3], // Public variable
});

impl GenericDefine<M31Config> for E6MulBy01Circuit<Variable> {
    fn define<Builder: RootAPI<M31Config>>(&self, builder: &mut Builder) {
        let mut ext6 = Ext6::new(builder);

        let a_e6 = GE6 {
            b0: GE2 {
                a0: new_internal_element(self.a[0][0].to_vec(), 0),
                a1: new_internal_element(self.a[0][1].to_vec(), 0),
            },
            b1: GE2 {
                a0: new_internal_element(self.a[1][0].to_vec(), 0),
                a1: new_internal_element(self.a[1][1].to_vec(), 0),
            },
            b2: GE2 {
                a0: new_internal_element(self.a[2][0].to_vec(), 0),
                a1: new_internal_element(self.a[2][1].to_vec(), 0),
            },
        };

        let c0_e2 = GE2 {
            a0: new_internal_element(self.c0[0].to_vec(), 0),
            a1: new_internal_element(self.c0[1].to_vec(), 0),
        };

        let c1_e2 = GE2 {
            a0: new_internal_element(self.c1[0].to_vec(), 0),
            a1: new_internal_element(self.c1[1].to_vec(), 0),
        };

        let c_e6 = GE6 {
            b0: GE2 {
                a0: new_internal_element(self.c[0][0].to_vec(), 0),
                a1: new_internal_element(self.c[0][1].to_vec(), 0),
            },
            b1: GE2 {
                a0: new_internal_element(self.c[1][0].to_vec(), 0),
                a1: new_internal_element(self.c[1][1].to_vec(), 0),
            },
            b2: GE2 {
                a0: new_internal_element(self.c[2][0].to_vec(), 0),
                a1: new_internal_element(self.c[2][1].to_vec(), 0),
            },
        };

        let result = ext6.mul_by_01(builder, &a_e6, &c0_e2, &c1_e2);

        ext6.assert_isequal(builder, &result, &c_e6);

        ext6.ext2.fp.check_mul(builder);
        ext6.ext2.fp.table.final_check(builder);
    }
}

#[test]
fn test_e6_mul_by_01() {
    // let compile_result =
    //     compile_generic(&E6MulBy01Circuit::default(), CompileOptions::default()).unwrap();
    let mut hint_registry = HintRegistry::<M31>::new();
    register_hint(&mut hint_registry);

    let mut assignment = E6MulBy01Circuit::<M31> {
        a: [[[M31::from(0); 48]; 2]; 3],
        c0: [[M31::from(0); 48]; 2],
        c1: [[M31::from(0); 48]; 2],
        c: [[[M31::from(0); 48]; 2]; 3],
    };
    let x0_b0_a0_bytes = [239,229,161,178,64,169,64,146,202,108,226,209,171,161,210,163,187,178,82,117,197,147,230,123,200,118,68,116,34,4,83,5,152,248,76,174,5,112,146,135,108,122,197,44,5,108,105,4,];
    let x0_b0_a1_bytes = [216,141,84,101,248,2,198,56,82,51,71,90,78,183,64,149,118,57,60,187,111,237,194,199,219,87,147,173,207,209,64,111,123,230,108,254,244,133,53,127,124,63,113,147,77,118,183,3,];
    let x0_b1_a0_bytes = [252,68,138,30,240,188,31,211,225,176,125,69,159,20,155,74,109,188,182,240,117,158,67,126,170,59,191,249,176,86,164,133,153,181,0,208,232,168,81,236,62,23,145,81,4,201,133,15,];
    let x0_b1_a1_bytes = [69,4,32,130,215,215,132,105,38,152,198,127,228,215,56,21,211,172,97,142,60,71,76,251,213,10,173,20,136,142,2,77,211,134,48,29,14,55,27,130,246,106,239,48,238,88,93,16,];
    let x0_b2_a0_bytes = [14,194,113,170,251,40,206,58,33,253,225,10,146,13,43,65,62,73,217,189,74,205,137,20,25,102,195,121,173,201,149,110,4,161,24,190,208,112,21,234,125,84,183,230,250,37,20,24,];
    let x0_b2_a1_bytes = [107,114,82,151,175,169,28,209,16,59,150,160,0,123,71,152,251,135,94,27,160,226,181,125,56,52,234,172,73,206,144,100,142,162,227,202,84,30,143,93,245,250,146,243,7,104,210,22,];
    let x1_a0_bytes = [186,151,19,68,40,192,201,108,0,91,94,25,135,234,188,37,171,13,192,227,215,174,77,246,206,150,192,189,188,18,52,109,174,255,45,7,112,19,158,246,207,176,139,230,213,125,252,17,];
    let x1_a1_bytes = [21,143,182,121,149,97,79,60,204,97,32,34,238,52,114,69,145,70,181,151,20,254,118,41,21,21,225,217,126,14,178,141,239,124,163,129,73,88,135,179,215,84,62,114,42,64,68,7,];
    let x2_a0_bytes = [138,88,211,80,5,54,126,91,234,136,231,41,212,67,79,189,64,69,62,2,130,218,241,195,164,151,141,15,73,243,223,243,185,165,89,79,139,227,17,201,244,9,196,252,155,229,41,14,];
    let x2_a1_bytes = [188,54,82,119,88,70,53,72,210,158,255,168,36,111,243,221,38,115,86,69,191,147,157,51,99,204,161,227,117,163,184,79,219,60,101,125,235,215,48,147,224,77,251,76,225,240,1,17,];
    let x3_b0_a0_bytes = [40,96,6,151,173,123,226,158,228,208,229,107,250,123,77,212,186,116,42,150,131,126,246,122,153,71,111,206,37,27,249,210,5,214,63,13,26,76,236,228,15,27,44,68,86,230,77,24,];
    let x3_b0_a1_bytes = [140,178,226,46,250,177,38,248,99,255,15,55,233,151,29,199,102,241,52,35,95,113,183,199,214,107,102,112,177,214,175,168,34,130,161,190,49,245,201,91,45,35,145,57,43,204,222,2,];
    let x3_b1_a0_bytes = [246,231,192,70,80,0,214,197,196,105,124,197,34,205,213,205,9,189,175,232,67,175,201,10,43,23,174,144,116,110,21,175,81,126,128,21,252,69,168,54,68,86,146,195,55,198,122,4,];
    let x3_b1_a1_bytes = [249,240,86,232,156,233,242,7,101,210,128,59,74,51,114,86,181,2,22,200,2,61,154,240,138,7,136,232,239,90,39,109,149,12,0,53,248,48,198,163,88,108,25,86,41,192,50,8,];
    let x3_b2_a0_bytes = [202,120,182,202,118,232,150,158,129,79,84,133,125,42,4,175,202,174,44,152,67,60,67,69,30,143,122,56,108,238,162,89,197,243,15,19,209,209,143,217,164,38,189,171,222,13,210,19,];
    let x3_b2_a1_bytes = [154,134,254,146,102,16,154,179,160,89,167,216,187,214,197,64,58,26,12,159,107,92,130,18,94,56,7,68,33,81,44,186,118,68,216,94,84,87,90,231,93,231,209,158,109,43,242,20,];

    for i in 0..48 {
        assignment.a[0][0][i] = M31::from(x0_b0_a0_bytes[i]);
        assignment.a[0][1][i] = M31::from(x0_b0_a1_bytes[i]);
        assignment.a[1][0][i] = M31::from(x0_b1_a0_bytes[i]);
        assignment.a[1][1][i] = M31::from(x0_b1_a1_bytes[i]);
        assignment.a[2][0][i] = M31::from(x0_b2_a0_bytes[i]);
        assignment.a[2][1][i] = M31::from(x0_b2_a1_bytes[i]);
        assignment.c0[0][i] = M31::from(x1_a0_bytes[i]);
        assignment.c0[1][i] = M31::from(x1_a1_bytes[i]);
        assignment.c1[0][i] = M31::from(x2_a0_bytes[i]);
        assignment.c1[1][i] = M31::from(x2_a1_bytes[i]);
        assignment.c[0][0][i] = M31::from(x3_b0_a0_bytes[i]);
        assignment.c[0][1][i] = M31::from(x3_b0_a1_bytes[i]);
        assignment.c[1][0][i] = M31::from(x3_b1_a0_bytes[i]);
        assignment.c[1][1][i] = M31::from(x3_b1_a1_bytes[i]);
        assignment.c[2][0][i] = M31::from(x3_b2_a0_bytes[i]);
        assignment.c[2][1][i] = M31::from(x3_b2_a1_bytes[i]);
    }

    debug_eval(&E6MulBy01Circuit::default(), &assignment, hint_registry);
}

declare_circuit!(E6NegCircuit {
    a: [[[Variable; 48]; 2]; 3],
    c: [[[Variable; 48]; 2]; 3], // Public variable
});

impl GenericDefine<M31Config> for E6NegCircuit<Variable> {
    fn define<Builder: RootAPI<M31Config>>(&self, builder: &mut Builder) {
        let mut ext6 = Ext6::new(builder);

        let a_e6 = GE6 {
            b0: GE2 {
                a0: new_internal_element(self.a[0][0].to_vec(), 0),
                a1: new_internal_element(self.a[0][1].to_vec(), 0),
            },
            b1: GE2 {
                a0: new_internal_element(self.a[1][0].to_vec(), 0),
                a1: new_internal_element(self.a[1][1].to_vec(), 0),
            },
            b2: GE2 {
                a0: new_internal_element(self.a[2][0].to_vec(), 0),
                a1: new_internal_element(self.a[2][1].to_vec(), 0),
            },
        };

        let c_e6 = GE6 {
            b0: GE2 {
                a0: new_internal_element(self.c[0][0].to_vec(), 0),
                a1: new_internal_element(self.c[0][1].to_vec(), 0),
            },
            b1: GE2 {
                a0: new_internal_element(self.c[1][0].to_vec(), 0),
                a1: new_internal_element(self.c[1][1].to_vec(), 0),
            },
            b2: GE2 {
                a0: new_internal_element(self.c[2][0].to_vec(), 0),
                a1: new_internal_element(self.c[2][1].to_vec(), 0),
            },
        };

        let result = ext6.neg(builder, &a_e6);
        ext6.assert_isequal(builder, &result, &c_e6);

        ext6.ext2.fp.check_mul(builder);
        ext6.ext2.fp.table.final_check(builder);
    }
}

#[test]
fn test_e6_neg() {
    let compile_result =
        compile_generic(&E6NegCircuit::default(), CompileOptions::default()).unwrap();
    let mut hint_registry = HintRegistry::<M31>::new();
    register_hint(&mut hint_registry);

    let mut assignment = E6NegCircuit::<M31> {
        a: [[[M31::from(0); 48]; 2]; 3],
        c: [[[M31::from(0); 48]; 2]; 3],
    };

    let x0_b0_a0_bytes = [116,6,234,253,168,74,65,30,170,142,158,184,33,84,176,59,39,31,68,152,100,233,15,176,94,80,69,58,137,167,36,189,51,230,84,91,111,236,115,231,37,185,220,160,17,14,196,3,];
    let x0_b0_a1_bytes = [74,217,113,27,10,53,174,157,74,32,126,65,73,185,191,214,75,202,59,40,1,229,87,54,182,214,172,205,241,238,156,6,115,105,1,134,107,190,214,227,195,156,125,3,27,177,68,20,];
    let x0_b1_a0_bytes = [156,1,181,29,159,51,200,2,179,151,250,205,64,17,207,162,7,246,108,213,210,159,81,251,163,6,43,23,100,250,77,164,96,61,201,255,155,157,17,183,138,30,232,18,210,234,119,13,];
    let x0_b1_a1_bytes = [67,74,29,124,15,39,125,211,85,255,163,176,37,195,144,76,67,69,116,59,54,163,254,137,168,252,55,64,225,163,218,46,91,93,133,23,105,178,144,210,71,102,22,156,220,31,126,3,];
    let x0_b2_a0_bytes = [165,53,235,67,200,212,135,127,103,241,184,182,61,98,13,112,24,61,180,73,29,81,249,63,111,128,12,220,3,213,244,214,126,148,142,13,20,84,97,163,244,109,32,173,58,146,143,23,];
    let x0_b2_a1_bytes = [139,176,170,247,65,42,233,157,160,227,93,104,151,125,167,9,117,73,194,2,23,230,150,90,203,142,63,12,47,48,180,119,136,117,87,9,48,16,188,215,25,173,239,70,235,131,89,12,];
    let x3_b0_a0_bytes = [55,164,21,2,87,181,189,155,85,113,181,248,220,171,251,226,252,214,108,94,60,233,32,183,96,194,63,185,251,163,82,167,163,198,246,231,70,187,167,99,116,45,163,152,216,3,61,22,];
    let x3_b0_a1_bytes = [97,209,141,228,245,202,80,28,181,223,213,111,181,70,236,71,216,43,117,206,159,237,216,48,9,60,216,37,147,92,218,93,100,67,74,189,74,233,68,103,214,73,2,54,207,96,188,5,];
    let x3_b1_a0_bytes = [15,169,74,226,96,204,54,183,76,104,89,227,189,238,220,123,28,0,68,33,206,50,223,107,27,12,90,220,32,81,41,192,118,111,130,67,26,10,10,148,15,200,151,38,24,39,137,12,];
    let x3_b1_a1_bytes = [104,96,226,131,240,216,129,230,169,0,176,0,217,60,27,210,224,176,60,187,106,47,50,221,22,22,77,179,163,167,156,53,124,79,198,43,77,245,138,120,82,128,105,157,13,242,130,22,];
    let x3_b2_a0_bytes = [6,117,20,188,55,43,119,58,152,14,155,250,192,157,158,174,11,185,252,172,131,129,55,39,80,146,120,23,129,118,130,141,88,24,189,53,162,83,186,167,165,120,95,140,175,127,113,2,];
    let x3_b2_a1_bytes = [32,250,84,8,190,213,21,28,95,28,246,72,103,130,4,21,175,172,238,243,137,236,153,12,244,131,69,231,85,27,195,236,78,55,244,57,134,151,95,115,128,57,144,242,254,141,167,13,];

    for i in 0..48 {
        assignment.a[0][0][i] = M31::from(x0_b0_a0_bytes[i]);
        assignment.a[0][1][i] = M31::from(x0_b0_a1_bytes[i]);
        assignment.a[1][0][i] = M31::from(x0_b1_a0_bytes[i]);
        assignment.a[1][1][i] = M31::from(x0_b1_a1_bytes[i]);
        assignment.a[2][0][i] = M31::from(x0_b2_a0_bytes[i]);
        assignment.a[2][1][i] = M31::from(x0_b2_a1_bytes[i]);
        assignment.c[0][0][i] = M31::from(x3_b0_a0_bytes[i]);
        assignment.c[0][1][i] = M31::from(x3_b0_a1_bytes[i]);
        assignment.c[1][0][i] = M31::from(x3_b1_a0_bytes[i]);
        assignment.c[1][1][i] = M31::from(x3_b1_a1_bytes[i]);
        assignment.c[2][0][i] = M31::from(x3_b2_a0_bytes[i]);
        assignment.c[2][1][i] = M31::from(x3_b2_a1_bytes[i]);
    }
    
    debug_eval(&E6NegCircuit::default(), &assignment, hint_registry);
}
declare_circuit!(E6InverseCircuit {
    a: [[[Variable; 48]; 2]; 3],
    c: [[[Variable; 48]; 2]; 3], // Public variable
});

impl GenericDefine<M31Config> for E6InverseCircuit<Variable> {
    fn define<Builder: RootAPI<M31Config>>(&self, builder: &mut Builder) {
        let mut ext6 = Ext6::new(builder);

        let a_e6 = GE6 {
            b0: GE2 {
                a0: new_internal_element(self.a[0][0].to_vec(), 0),
                a1: new_internal_element(self.a[0][1].to_vec(), 0),
            },
            b1: GE2 {
                a0: new_internal_element(self.a[1][0].to_vec(), 0),
                a1: new_internal_element(self.a[1][1].to_vec(), 0),
            },
            b2: GE2 {
                a0: new_internal_element(self.a[2][0].to_vec(), 0),
                a1: new_internal_element(self.a[2][1].to_vec(), 0),
            },
        };

        let c_e6 = GE6 {
            b0: GE2 {
                a0: new_internal_element(self.c[0][0].to_vec(), 0),
                a1: new_internal_element(self.c[0][1].to_vec(), 0),
            },
            b1: GE2 {
                a0: new_internal_element(self.c[1][0].to_vec(), 0),
                a1: new_internal_element(self.c[1][1].to_vec(), 0),
            },
            b2: GE2 {
                a0: new_internal_element(self.c[2][0].to_vec(), 0),
                a1: new_internal_element(self.c[2][1].to_vec(), 0),
            },
        };

        let result = ext6.inverse(builder, &a_e6);
        ext6.assert_isequal(builder, &result, &c_e6);

        ext6.ext2.fp.check_mul(builder);
        ext6.ext2.fp.table.final_check(builder);
    }
}

#[test]
fn test_e6_inverse() {
    let compile_result =
        compile_generic(&E6InverseCircuit::default(), CompileOptions::default()).unwrap();
    let mut hint_registry = HintRegistry::<M31>::new();
    register_hint(&mut hint_registry);

    let mut assignment = E6InverseCircuit::<M31> {
        a: [[[M31::from(0); 48]; 2]; 3],
        c: [[[M31::from(0); 48]; 2]; 3],
    };

    let x0_b0_a0_bytes = [42,191,107,1,61,26,173,13,160,78,61,122,92,29,163,162,133,224,146,25,59,158,4,106,41,66,220,84,62,148,251,247,116,66,190,14,209,79,118,179,163,124,142,157,70,75,135,9,];
    let x0_b0_a1_bytes = [211,80,115,82,164,221,106,133,199,205,208,188,168,21,57,40,179,134,122,83,214,125,232,227,94,208,153,53,5,91,60,107,111,192,42,241,126,4,223,7,202,41,151,248,42,136,202,18,];
    let x0_b1_a0_bytes = [230,142,241,182,172,53,243,38,51,114,207,39,193,178,94,164,237,60,49,201,56,151,44,35,115,180,149,238,95,234,223,68,115,48,95,57,92,8,2,55,89,227,203,32,236,8,37,8,];
    let x0_b1_a1_bytes = [175,204,91,4,54,39,255,219,210,131,129,250,20,29,26,195,225,84,161,62,19,4,156,203,236,158,167,164,177,156,156,191,39,168,77,57,213,134,75,249,148,206,186,177,237,248,25,20,];
    let x0_b2_a0_bytes = [72,70,59,131,175,200,39,60,247,77,55,65,105,174,197,3,147,15,56,34,225,101,126,71,117,222,105,147,48,91,61,157,29,199,238,20,87,18,143,164,207,65,151,173,84,221,69,8,];
    let x0_b2_a1_bytes = [124,176,9,207,196,159,159,65,67,227,130,231,59,74,160,145,206,84,167,199,54,98,13,14,88,232,246,1,134,251,196,191,209,208,89,19,159,83,100,169,65,148,60,147,220,58,39,10,];
    let x3_b0_a0_bytes = [241,211,96,221,135,252,51,160,240,44,177,6,233,34,43,65,225,187,89,228,132,88,152,212,254,70,210,244,133,61,76,202,1,214,152,159,50,108,226,224,77,138,58,52,196,171,248,2,];
    let x3_b0_a1_bytes = [102,158,6,155,253,105,81,12,177,99,91,215,140,62,35,12,235,225,229,225,110,51,146,31,209,37,204,124,153,134,139,92,185,55,128,182,137,140,126,70,213,91,217,27,245,2,135,12,];
    let x3_b1_a0_bytes = [80,250,232,255,129,150,236,243,241,211,26,29,138,145,205,240,56,146,126,65,224,117,109,179,85,61,139,201,97,176,208,180,213,192,135,20,113,168,90,174,215,144,185,63,18,118,199,16,];
    let x3_b1_a1_bytes = [79,99,136,50,88,106,124,92,158,146,150,211,235,118,143,132,238,206,182,239,228,54,55,88,72,112,177,56,58,73,253,9,218,106,84,202,167,194,137,34,248,71,70,206,63,56,27,6,];
    let x3_b2_a0_bytes = [214,90,220,213,91,247,245,183,117,178,27,175,136,232,144,62,52,5,23,96,176,81,121,179,19,91,112,174,163,162,230,68,126,148,42,157,89,88,68,113,249,197,123,86,231,35,229,21,];
    let x3_b2_a1_bytes = [138,250,218,214,205,57,171,168,67,27,229,167,87,177,26,86,82,57,100,97,198,239,162,172,62,30,46,232,182,101,113,253,139,213,76,44,222,32,201,43,244,235,1,22,14,141,123,25,];

    for i in 0..48 {
        assignment.a[0][0][i] = M31::from(x0_b0_a0_bytes[i]);
        assignment.a[0][1][i] = M31::from(x0_b0_a1_bytes[i]);
        assignment.a[1][0][i] = M31::from(x0_b1_a0_bytes[i]);
        assignment.a[1][1][i] = M31::from(x0_b1_a1_bytes[i]);
        assignment.a[2][0][i] = M31::from(x0_b2_a0_bytes[i]);
        assignment.a[2][1][i] = M31::from(x0_b2_a1_bytes[i]);
        assignment.c[0][0][i] = M31::from(x3_b0_a0_bytes[i]);
        assignment.c[0][1][i] = M31::from(x3_b0_a1_bytes[i]);
        assignment.c[1][0][i] = M31::from(x3_b1_a0_bytes[i]);
        assignment.c[1][1][i] = M31::from(x3_b1_a1_bytes[i]);
        assignment.c[2][0][i] = M31::from(x3_b2_a0_bytes[i]);
        assignment.c[2][1][i] = M31::from(x3_b2_a1_bytes[i]);
    }

    debug_eval(&E6InverseCircuit::default(), &assignment, hint_registry);
}


// pub fn print_e2<'a, C:Config, B:RootAPI<C>>(native: &'a mut B, v: &GE2)  {
//     for i in 0..48 {
//         println!("{}: {:?} {:?}", i, native.value_of(v.a0.limbs[i]), native.value_of(v.a1.limbs[i]));
//     }
// }
// pub fn print_element<'a, C:Config, B:RootAPI<C>, T: FieldParams>(native: &'a mut B, v: &Element<T>)  {
//     // for i in 0..48 {
//     //     print!("{}: {:?}", i, native.value_of(v.limbs[i]));
//     // }
//     // println!();
// }