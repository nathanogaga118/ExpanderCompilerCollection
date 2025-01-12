use crate::gnark::element::*;
use crate::gnark::hints::register_hint;
use expander_compiler::frontend::extra::*;
use expander_compiler::frontend::*;

use super::e2::*;
use super::e6::*;
#[derive(Default, Clone)]
pub struct GE12 {
    pub c0: GE6,
    pub c1: GE6,
}
impl GE12 {
    pub fn clone(&self) -> Self {
        GE12 {
            c0: self.c0.clone(),
            c1: self.c1.clone(),
        }
    }
}
pub struct Ext12 {
    pub ext6: Ext6,
}

impl Ext12{
    pub fn new<'a, C:Config, B:RootAPI<C>>(api: &'a mut B) -> Self {
        Self {
            ext6: Ext6::new(api),
        }
    }
    pub fn zero(&mut self) -> GE12 {
        let zero = self.ext6.ext2.fp.zero_const.clone();
        GE12 {
            c0: GE6 {
                b0: GE2 {
                    a0: zero.clone(),
                    a1: zero.clone(),
                },
                b1: GE2 {
                    a0: zero.clone(),
                    a1: zero.clone(),
                },
                b2: GE2 {
                    a0: zero.clone(),
                    a1: zero.clone(),
                },
            },
            c1: GE6 {
                b0: GE2 {
                    a0: zero.clone(),
                    a1: zero.clone(),
                },
                b1: GE2 {
                    a0: zero.clone(),
                    a1: zero.clone(),
                },
                b2: GE2 {
                    a0: zero.clone(),
                    a1: zero.clone(),
                },
            },
        }
    }
    pub fn one(&mut self) -> GE12 {
        let one = self.ext6.ext2.fp.one_const.clone();
        let zero = self.ext6.ext2.fp.zero_const.clone();
        GE12 {
            c0: GE6 {
                b0: GE2 {
                    a0: one.clone(),
                    a1: zero.clone(),
                },
                b1: GE2 {
                    a0: zero.clone(),
                    a1: zero.clone(),
                },
                b2: GE2 {
                    a0: zero.clone(),
                    a1: zero.clone(),
                },
            },
            c1: GE6 {
                b0: GE2 {
                    a0: zero.clone(),
                    a1: zero.clone(),
                },
                b1: GE2 {
                    a0: zero.clone(),
                    a1: zero.clone(),
                },
                b2: GE2 {
                    a0: zero.clone(),
                    a1: zero.clone(),
                },
            },
        }
    }
    pub fn is_zero<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, z: &GE12) -> Variable {
        let c0 = self.ext6.is_zero(native, &z.c0);
        let c1 = self.ext6.is_zero(native, &z.c1);
        native.and(c0, c1)
    }
    pub fn add<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE12, y: &GE12) -> GE12 {
        let z0 = self.ext6.add(native, &x.c0, &y.c0);
        let z1 = self.ext6.add(native, &x.c1, &y.c1);
        GE12 {
            c0: z0,
            c1: z1,
        }
    }
    pub fn sub<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE12, y: &GE12) -> GE12 {
        let z0 = self.ext6.sub(native, &x.c0, &y.c0);
        let z1 = self.ext6.sub(native, &x.c1, &y.c1);
        GE12 {
            c0: z0,
            c1: z1,
        }
    }
    pub fn conjugate<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE12) -> GE12 {
        let z1 = self.ext6.neg(native, &x.c1);
        GE12 {
            c0: x.c0.clone(),
            c1: z1,
        }
    }
    pub fn mul<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE12, y: &GE12) -> GE12 {
        let a = self.ext6.add(native, &x.c0, &x.c1);
        let b = self.ext6.add(native, &y.c0, &y.c1);
        let a = self.ext6.mul(native, &a, &b);
        let b = self.ext6.mul(native, &x.c0, &y.c0);
        let c = self.ext6.mul(native, &x.c1, &y.c1);
        let d = self.ext6.add(native, &c, &b);
        let z1 = self.ext6.sub(native, &a, &d);
        let z0 = self.ext6.mul_by_non_residue(native, &c);
        let z0 = self.ext6.add(native, &z0, &b);
        GE12 {
            c0: z0,
            c1: z1,
        }
    }
    pub fn square<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE12) -> GE12 {
        let c0 = self.ext6.sub(native, &x.c0, &x.c1);
        let c3 = self.ext6.mul_by_non_residue(native, &x.c1);
        let c3 = self.ext6.sub(native, &x.c0, &c3);
        let c2 = self.ext6.mul(native, &x.c0, &x.c1);
        let c0 = self.ext6.mul(native, &c0, &c3);
        let c0 = self.ext6.add(native, &c0, &c2);
        let z1 = self.ext6.double(native, &c2);
        let c2 = self.ext6.mul_by_non_residue(native, &c2);
        let z0 = self.ext6.add(native, &c0, &c2);
        GE12 {
            c0: z0,
            c1: z1,
        }
    }

    pub fn cyclotomic_square<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE12) -> GE12 {
        let t0 = self.ext6.ext2.square(native, &x.c1.b1);
        let t1 = self.ext6.ext2.square(native, &x.c0.b0);
        let mut t6 = self.ext6.ext2.add(native, &x.c1.b1, &x.c0.b0);
        t6 = self.ext6.ext2.square(native, &t6);
        t6 = self.ext6.ext2.sub(native, &t6, &t0);
        t6 = self.ext6.ext2.sub(native, &t6, &t1);
        let t2 = self.ext6.ext2.square(native, &x.c0.b2);
        let t3 = self.ext6.ext2.square(native, &x.c1.b0);
        let mut t7 = self.ext6.ext2.add(native, &x.c0.b2, &x.c1.b0);
        t7 = self.ext6.ext2.square(native, &t7);
        t7 = self.ext6.ext2.sub(native, &t7, &t2);
        t7 = self.ext6.ext2.sub(native, &t7, &t3);
        let t4 = self.ext6.ext2.square(native, &x.c1.b2);
        let t5 = self.ext6.ext2.square(native, &x.c0.b1);
        let mut t8 = self.ext6.ext2.add(native, &x.c1.b2, &x.c0.b1);
        t8 = self.ext6.ext2.square(native, &t8);
        t8 = self.ext6.ext2.sub(native, &t8, &t4);
        t8 = self.ext6.ext2.sub(native, &t8, &t5);
        t8 = self.ext6.ext2.mul_by_non_residue(native, &t8);
        let t0 = self.ext6.ext2.mul_by_non_residue(native, &t0);
        let t0 = self.ext6.ext2.add(native, &t0, &t1);
        let t2 = self.ext6.ext2.mul_by_non_residue(native, &t2);
        let t2 = self.ext6.ext2.add(native, &t2, &t3);
        let t4 = self.ext6.ext2.mul_by_non_residue(native, &t4);
        let t4 = self.ext6.ext2.add(native, &t4, &t5);
        let z00 = self.ext6.ext2.sub(native, &t0, &x.c0.b0);
        let z00 = self.ext6.ext2.double(native, &z00);
        let z00 = self.ext6.ext2.add(native, &z00, &t0);
        let z01 = self.ext6.ext2.sub(native, &t2, &x.c0.b1);
        let z01 = self.ext6.ext2.double(native, &z01);
        let z01 = self.ext6.ext2.add(native, &z01, &t2);
        let z02 = self.ext6.ext2.sub(native, &t4, &x.c0.b2);
        let z02 = self.ext6.ext2.double(native, &z02);
        let z02 = self.ext6.ext2.add(native, &z02, &t4);
        let z10 = self.ext6.ext2.add(native, &t8, &x.c1.b0);
        let z10 = self.ext6.ext2.double(native, &z10);
        let z10 = self.ext6.ext2.add(native, &z10, &t8);
        let z11 = self.ext6.ext2.add(native, &t6, &x.c1.b1);
        let z11 = self.ext6.ext2.double(native, &z11);
        let z11 = self.ext6.ext2.add(native, &z11, &t6);
        let z12 = self.ext6.ext2.add(native, &t7, &x.c1.b2);
        let z12 = self.ext6.ext2.double(native, &z12);
        let z12 = self.ext6.ext2.add(native, &z12, &t7);
        GE12 {
            c0: GE6 {
                b0: z00,
                b1: z01,
                b2: z02,
            },
            c1: GE6 {
                b0: z10,
                b1: z11,
                b2: z12,
            },
        }
    }
    pub fn assert_isequal<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE12, y: &GE12) {
        self.ext6.assert_isequal(native, &x.c0, &y.c0);
        self.ext6.assert_isequal(native, &x.c1, &y.c1);
    }
    pub fn div<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE12, y: &GE12) -> GE12 {
        let inputs = vec![x.c0.b0.a0.clone(), x.c0.b0.a1.clone(), x.c0.b1.a0.clone(), x.c0.b1.a1.clone(), x.c0.b2.a0.clone(), x.c0.b2.a1.clone(), x.c1.b0.a0.clone(), x.c1.b0.a1.clone(), x.c1.b1.a0.clone(), x.c1.b1.a1.clone(), x.c1.b2.a0.clone(), x.c1.b2.a1.clone(), y.c0.b0.a0.clone(), y.c0.b0.a1.clone(), y.c0.b1.a0.clone(), y.c0.b1.a1.clone(), y.c0.b2.a0.clone(), y.c0.b2.a1.clone(), y.c1.b0.a0.clone(), y.c1.b0.a1.clone(), y.c1.b1.a0.clone(), y.c1.b1.a1.clone(), y.c1.b2.a0.clone(), y.c1.b2.a1.clone()];
        let output = self.ext6.ext2.fp.new_hint(native, "myhint.dive12hint", 24, inputs);
        let div = GE12 {
            c0: GE6 {
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
            },
            c1: GE6 {
                b0: GE2 {
                    a0: output[6].clone(),
                    a1: output[7].clone(),
                },
                b1: GE2 {
                    a0: output[8].clone(),
                    a1: output[9].clone(),
                },
                b2: GE2 {
                    a0: output[10].clone(),
                    a1: output[11].clone(),
                },
            },
        };
        let _x = self.mul(native, &div, y);
        self.assert_isequal(native, x, &_x);
        div
    }
    pub fn inverse<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE12) -> GE12 {
        let inputs = vec![x.c0.b0.a0.clone(), x.c0.b0.a1.clone(), x.c0.b1.a0.clone(), x.c0.b1.a1.clone(), x.c0.b2.a0.clone(), x.c0.b2.a1.clone(), x.c1.b0.a0.clone(), x.c1.b0.a1.clone(), x.c1.b1.a0.clone(), x.c1.b1.a1.clone(), x.c1.b2.a0.clone(), x.c1.b2.a1.clone()];
        let output = self.ext6.ext2.fp.new_hint(native, "myhint.inversee12hint", 12, inputs);
        let inv = GE12 {
            c0: GE6 {
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
            },
            c1: GE6 {
                b0: GE2 {
                    a0: output[6].clone(),
                    a1: output[7].clone(),
                },
                b1: GE2 {
                    a0: output[8].clone(),
                    a1: output[9].clone(),
                },
                b2: GE2 {
                    a0: output[10].clone(),
                    a1: output[11].clone(),
                },
            },
        };
        let one = self.one();
        let _one = self.mul(native, &inv, x);
        self.assert_isequal(native, &one, &_one);
        inv
    }
    pub fn copy<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE12) -> GE12 {
        let inputs = vec![x.c0.b0.a0.clone(), x.c0.b0.a1.clone(), x.c0.b1.a0.clone(), x.c0.b1.a1.clone(), x.c0.b2.a0.clone(), x.c0.b2.a1.clone(), x.c1.b0.a0.clone(), x.c1.b0.a1.clone(), x.c1.b1.a0.clone(), x.c1.b1.a1.clone(), x.c1.b2.a0.clone(), x.c1.b2.a1.clone()];
        let output = self.ext6.ext2.fp.new_hint(native, "myhint.copye12hint", 12, inputs);
        let res = GE12 {
            c0: GE6 {
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
            },
            c1: GE6 {
                b0: GE2 {
                    a0: output[6].clone(),
                    a1: output[7].clone(),
                },
                b1: GE2 {
                    a0: output[8].clone(),
                    a1: output[9].clone(),
                },
                b2: GE2 {
                    a0: output[10].clone(),
                    a1: output[11].clone(),
                },
            },
        };
        self.assert_isequal(native, x, &res);
        res
    }
    pub fn select<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, selector: Variable, z1: &GE12, z0: &GE12) -> GE12 {
        let c0 = self.ext6.select(native, selector, &z1.c0, &z0.c0);
        let c1 = self.ext6.select(native, selector, &z1.c1, &z0.c1);
        GE12 {
            c0: c0,
            c1: c1,
        }
    }

    /////// pairing ///////
    pub fn mul_by_014<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, z: &GE12, c0: &GE2, c1: &GE2) -> GE12 {
        let a = self.ext6.mul_by_01(native, &z.c0, c0, c1);
        let b = GE6 {
            b0: self.ext6.ext2.mul_by_non_residue(native, &z.c1.b2),
            b1: z.c1.b0.clone(),
            b2: z.c1.b1.clone(),
        };
        let one = self.ext6.ext2.one();
        let d = self.ext6.ext2.add(native, c1, &one);
        let zc1 = self.ext6.add(native, &z.c1, &z.c0);
        let zc1 = self.ext6.mul_by_01(native, &zc1, c0, &d);
        let tmp = self.ext6.add(native, &b, &a);
        let zc1 = self.ext6.sub(native, &zc1, &tmp);
        let zc0 = self.ext6.mul_by_non_residue(native, &b);
        let zc0 = self.ext6.add(native, &zc0, &a);
        GE12 {
            c0: zc0,
            c1: zc1,
        }
    }
    pub fn mul_014_by_014<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, d0: &GE2, d1: &GE2, c0: &GE2, c1: &GE2) -> [GE2; 5] {
        let x0 = self.ext6.ext2.mul(native, c0, d0);
        let x1 = self.ext6.ext2.mul(native, c1, d1);
        let x04 = self.ext6.ext2.add(native, c0, d0);
        let tmp = self.ext6.ext2.add(native, c0, c1);
        let x01 = self.ext6.ext2.add(native, d0, d1);
        let x01 = self.ext6.ext2.mul(native, &x01, &tmp);
        let tmp = self.ext6.ext2.add(native, &x1, &x0);
        let x01 = self.ext6.ext2.sub(native, &x01, &tmp);
        let x14 = self.ext6.ext2.add(native, c1, d1);
        let z_c0_b0 = self.ext6.ext2.non_residue(native);
        let z_c0_b0 = self.ext6.ext2.add(native, &z_c0_b0, &x0);
        [z_c0_b0, x01, x1, x04, x14]
    }
    pub fn expt<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE12) -> GE12 {
        let z = self.cyclotomic_square(native, x);
        let z = self.mul(native, x, &z);
        let z = self.n_square_gs_with_hint(native, &z, 2);
        let z = self.mul(native, x, &z);
        let z = self.n_square_gs_with_hint(native, &z, 3);
        let z = self.mul(native, x, &z);
        let z = self.n_square_gs_with_hint(native, &z, 9);
        let z = self.mul(native, x, &z);
        let z = self.n_square_gs_with_hint(native, &z, 32);
        let z = self.mul(native, x, &z);
        let z = self.n_square_gs_with_hint(native, &z, 15);
        let z = self.cyclotomic_square(native, &z);
        z
    }
    pub fn n_square_gs<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, z: &GE12, n: usize) -> GE12 {
        let mut new_z = z.clone();
        for _ in 0..n {
            new_z = self.cyclotomic_square(native, &new_z);
        }
        new_z
    }
    pub fn n_square_gs_with_hint<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, z: &GE12, n: usize) -> GE12 {
        let mut copy_z = self.copy(native, z);
        for _ in 0..n-1 {
            let z = self.cyclotomic_square(native, &copy_z);
            copy_z = self.copy(native, &z);
        }
        self.cyclotomic_square(native, &copy_z)
    }
    pub fn assert_final_exponentiation_is_one<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE12) {
        let inputs = vec![x.c0.b0.a0.clone(), x.c0.b0.a1.clone(), x.c0.b1.a0.clone(), x.c0.b1.a1.clone(), x.c0.b2.a0.clone(), x.c0.b2.a1.clone(), x.c1.b0.a0.clone(), x.c1.b0.a1.clone(), x.c1.b1.a0.clone(), x.c1.b1.a1.clone(), x.c1.b2.a0.clone(), x.c1.b2.a1.clone()];
        let output = self.ext6.ext2.fp.new_hint(native, "myhint.finalexphint", 18, inputs);
        let residue_witness = GE12 {
            c0: GE6 {
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
            },
            c1: GE6 {
                b0: GE2 {
                    a0: output[6].clone(),
                    a1: output[7].clone(),
                },
                b1: GE2 {
                    a0: output[8].clone(),
                    a1: output[9].clone(),
                },
                b2: GE2 {
                    a0: output[10].clone(),
                    a1: output[11].clone(),
                },
            },
        };
        let scaling_factor = GE12 {
            c0: GE6 {
                b0: GE2 {
                    a0: output[12].clone(),
                    a1: output[13].clone(),
                },
                b1: GE2 {
                    a0: output[14].clone(),
                    a1: output[15].clone(),
                },
                b2: GE2 {
                    a0: output[16].clone(),
                    a1: output[17].clone(),
                },
            },
            c1: self.zero().c1,
        };
        let t0 = self.frobenius(native, &residue_witness);
        let t1 = self.expt(native, &residue_witness);
        let t0 = self.mul(native, &t0, &t1);
        let t1 = self.mul(native, x, &scaling_factor);
        self.assert_isequal(native, &t0, &t1);
    }

    pub fn frobenius<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE12) -> GE12 {
        let z00 = self.ext6.ext2.conjugate(native, &x.c0.b0);
        let z01 = self.ext6.ext2.conjugate(native, &x.c0.b1);
        let z02 = self.ext6.ext2.conjugate(native, &x.c0.b2);
        let z10 = self.ext6.ext2.conjugate(native, &x.c1.b0);
        let z11 = self.ext6.ext2.conjugate(native, &x.c1.b1);
        let z12 = self.ext6.ext2.conjugate(native, &x.c1.b2);

        let z01 = self.ext6.ext2.mul_by_non_residue1_power2(native, &z01);
        let z02 = self.ext6.ext2.mul_by_non_residue1_power4(native, &z02);
        let z10 = self.ext6.ext2.mul_by_non_residue1_power1(native, &z10);
        let z11 = self.ext6.ext2.mul_by_non_residue1_power3(native, &z11);
        let z12 = self.ext6.ext2.mul_by_non_residue1_power5(native, &z12);
        GE12 {
            c0: GE6 {
                b0: z00,
                b1: z01,
                b2: z02,
            },
            c1: GE6 {
                b0: z10,
                b1: z11,
                b2: z12,
            },
        }
    }
    pub fn frobenius_square<'a, C:Config, B:RootAPI<C>>(&mut self, native: &'a mut B, x: &GE12) -> GE12 {
        let z00 = x.c0.b0.clone();
        let z01 = self.ext6.ext2.mul_by_non_residue2_power2(native, &x.c0.b1);
        let z02 = self.ext6.ext2.mul_by_non_residue2_power4(native, &x.c0.b2);
        let z10 = self.ext6.ext2.mul_by_non_residue2_power1(native, &x.c1.b0);
        let z11 = self.ext6.ext2.mul_by_non_residue2_power3(native, &x.c1.b1);
        let z12 = self.ext6.ext2.mul_by_non_residue2_power5(native, &x.c1.b2);
        GE12 {
            c0: GE6 {
                b0: z00,
                b1: z01,
                b2: z02,
            },
            c1: GE6 {
                b0: z10,
                b1: z11,
                b2: z12,
            },
        }
    }
}


declare_circuit!(E12AddCircuit {
    x: [[[[Variable; 48];2];3];2],
    y: [[[[Variable; 48];2];3];2],
    z: [[[[Variable; 48];2];3];2],
});

impl GenericDefine<M31Config> for E12AddCircuit<Variable> {
    fn define<Builder: RootAPI<M31Config>>(&self, builder: &mut Builder) {
        let mut ext12 = Ext12::new(builder);
        let x_e12 = GE12 {
            c0: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.x[0][0][0].to_vec(), 0),
                    a1: new_internal_element(self.x[0][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.x[0][1][0].to_vec(), 0),
                    a1: new_internal_element(self.x[0][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.x[0][2][0].to_vec(), 0),
                    a1: new_internal_element(self.x[0][2][1].to_vec(), 0),
                },
            },
            c1: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.x[1][0][0].to_vec(), 0),
                    a1: new_internal_element(self.x[1][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.x[1][1][0].to_vec(), 0),
                    a1: new_internal_element(self.x[1][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.x[1][2][0].to_vec(), 0),
                    a1: new_internal_element(self.x[1][2][1].to_vec(), 0),
                },
            },
        };
        let y_e12 = GE12 {
            c0: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.y[0][0][0].to_vec(), 0),
                    a1: new_internal_element(self.y[0][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.y[0][1][0].to_vec(), 0),
                    a1: new_internal_element(self.y[0][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.y[0][2][0].to_vec(), 0),
                    a1: new_internal_element(self.y[0][2][1].to_vec(), 0),
                },
            },
            c1: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.y[1][0][0].to_vec(), 0),
                    a1: new_internal_element(self.y[1][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.y[1][1][0].to_vec(), 0),
                    a1: new_internal_element(self.y[1][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.y[1][2][0].to_vec(), 0),
                    a1: new_internal_element(self.y[1][2][1].to_vec(), 0),
                },
            },
        };
        let z_e12 = GE12 {
            c0: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.z[0][0][0].to_vec(), 0),
                    a1: new_internal_element(self.z[0][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.z[0][1][0].to_vec(), 0),
                    a1: new_internal_element(self.z[0][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.z[0][2][0].to_vec(), 0),
                    a1: new_internal_element(self.z[0][2][1].to_vec(), 0),
                },
            },
            c1: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.z[1][0][0].to_vec(), 0),
                    a1: new_internal_element(self.z[1][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.z[1][1][0].to_vec(), 0),
                    a1: new_internal_element(self.z[1][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.z[1][2][0].to_vec(), 0),
                    a1: new_internal_element(self.z[1][2][1].to_vec(), 0),
                },
            },
        };
        let z = ext12.add(builder, &x_e12, &y_e12);
        ext12.assert_isequal(builder, &z, &z_e12);
        ext12.ext6.ext2.fp.check_mul(builder);
        ext12.ext6.ext2.fp.table.final_check(builder);
    }
}
#[test]
fn test_e12_add() {
    let compile_result =
    compile_generic(&E12AddCircuit::default(), CompileOptions::default()).unwrap();
	let mut hint_registry = HintRegistry::<M31>::new();
    register_hint(&mut hint_registry);
    let mut assignment = E12AddCircuit::<M31> {
        x: [[[[M31::from(0); 48]; 2]; 3];2],
        y: [[[[M31::from(0); 48]; 2]; 3];2],
        z: [[[[M31::from(0); 48]; 2]; 3];2],
    };

    let x0_c0_b0_a0_bytes = [230,7,244,92,237,70,117,94,82,55,74,196,172,118,86,33,195,231,218,215,169,200,47,95,2,162,203,215,88,27,146,255,185,205,74,164,252,251,241,36,112,228,157,87,122,78,189,18,];
    let x0_c0_b0_a1_bytes = [123,74,33,121,6,155,7,109,108,65,144,138,43,39,102,201,193,139,222,60,96,210,211,212,214,250,64,56,217,19,222,230,161,139,175,92,207,204,60,236,42,23,130,36,116,94,235,22,];
    let x0_c0_b1_a0_bytes = [49,127,28,75,52,125,232,138,94,244,108,5,97,129,205,223,92,250,249,164,70,188,87,59,88,120,208,94,48,41,13,251,243,5,118,105,177,148,29,54,156,135,64,151,157,0,119,7,];
    let x0_c0_b1_a1_bytes = [111,133,18,247,78,21,80,154,216,230,186,223,109,228,163,119,98,30,52,145,174,146,135,230,44,58,58,70,56,108,96,150,67,181,53,124,38,92,190,174,68,18,176,112,232,23,102,7,];
    let x0_c0_b2_a0_bytes = [194,50,236,56,30,253,216,230,252,43,62,251,37,124,173,107,236,62,190,121,225,13,255,152,137,221,37,23,178,16,232,244,15,29,1,229,201,43,27,85,173,191,250,2,43,39,206,12,];
    let x0_c0_b2_a1_bytes = [141,208,78,212,20,209,73,151,224,146,235,177,88,38,231,36,205,8,223,66,35,157,28,37,123,92,239,77,190,243,142,2,228,145,241,47,251,55,59,116,195,196,90,86,171,39,236,12,];
    let x0_c1_b0_a0_bytes = [169,135,2,13,240,185,47,225,235,154,118,30,95,163,223,25,184,76,152,231,206,120,67,227,223,228,226,172,134,24,174,108,8,21,235,122,63,78,129,226,8,205,153,206,152,214,164,12,];
    let x0_c1_b0_a1_bytes = [250,192,145,229,203,199,112,129,255,241,90,53,11,91,241,117,135,247,116,237,193,5,104,198,55,136,215,148,136,67,185,172,209,102,122,64,180,67,152,220,92,166,177,36,137,82,210,4,];
    let x0_c1_b1_a0_bytes = [86,8,54,207,80,124,211,250,195,16,41,225,151,234,74,235,6,80,128,23,208,150,90,168,123,66,153,230,12,192,202,28,163,221,28,76,58,73,101,1,243,250,133,26,228,172,88,12,];
    let x0_c1_b1_a1_bytes = [100,82,131,139,164,216,135,48,179,232,54,9,39,131,147,137,241,60,21,218,161,102,144,134,81,101,64,0,5,131,214,170,224,123,11,25,160,89,220,166,193,45,13,100,230,116,112,24,];
    let x0_c1_b2_a0_bytes = [247,221,42,90,51,107,26,120,49,75,158,9,75,55,71,121,59,126,96,1,14,248,253,151,143,29,83,249,204,94,105,120,21,8,170,27,117,166,25,117,119,196,147,115,60,10,53,13,];
    let x0_c1_b2_a1_bytes = [46,173,19,115,230,103,157,253,229,42,46,181,62,74,133,99,144,63,196,246,4,132,203,228,77,114,70,247,63,15,138,100,9,32,145,80,245,98,110,218,156,33,57,62,43,98,81,18,];
    let x1_c0_b0_a0_bytes = [148,30,71,204,89,128,39,211,200,173,12,53,49,151,93,248,122,184,53,28,126,17,19,194,199,192,84,54,197,99,7,123,243,77,94,235,77,57,176,95,211,166,170,169,219,136,143,16,];
    let x1_c0_b0_a1_bytes = [116,165,190,228,91,60,196,159,85,252,213,69,1,2,255,229,48,82,242,236,138,116,18,142,211,226,1,27,172,39,110,176,116,224,29,170,150,162,188,133,134,187,63,39,42,233,223,21,];
    let x1_c0_b1_a0_bytes = [52,188,3,110,86,230,166,129,55,12,222,175,157,177,232,228,128,150,69,11,254,146,229,48,88,212,25,142,49,186,136,155,251,188,234,79,116,72,200,26,16,2,44,141,51,243,107,25,];
    let x1_c0_b1_a1_bytes = [189,11,14,178,64,171,213,99,42,92,224,19,135,91,69,10,17,74,95,100,229,165,14,89,76,7,26,12,141,254,74,178,222,63,209,235,231,191,198,239,111,184,20,119,247,206,137,21,];
    let x1_c0_b2_a0_bytes = [212,172,221,198,21,214,123,10,204,162,176,184,103,196,108,104,238,168,120,68,50,179,148,56,3,150,2,153,240,153,144,156,154,0,122,112,38,167,188,90,58,54,253,203,30,18,116,22,];
    let x1_c0_b2_a1_bytes = [90,124,114,30,19,47,172,69,32,76,109,59,202,137,251,14,81,116,190,33,48,205,103,135,26,77,174,125,197,102,92,138,15,20,230,7,205,140,129,234,229,245,234,158,122,90,136,20,];
    let x1_c1_b0_a0_bytes = [200,82,45,114,38,64,114,217,14,159,26,201,98,79,228,4,175,96,242,120,46,134,147,59,150,169,115,61,246,17,80,231,88,50,192,43,236,13,195,51,88,2,150,109,127,175,212,11,];
    let x1_c1_b0_a1_bytes = [90,205,64,128,120,157,119,255,181,86,183,85,39,214,168,122,184,70,236,137,17,168,133,48,19,22,156,44,154,42,65,94,10,74,77,91,168,172,235,220,114,60,8,25,65,146,138,10,];
    let x1_c1_b1_a0_bytes = [79,42,100,15,28,174,145,214,133,51,126,38,14,120,235,155,26,216,119,134,149,230,93,241,130,50,39,124,254,144,244,88,224,222,252,49,70,167,245,170,157,178,32,1,188,90,249,25,];
    let x1_c1_b1_a1_bytes = [23,37,23,6,168,183,104,99,161,213,146,108,40,203,206,138,143,9,137,68,6,6,215,212,160,97,220,1,20,120,149,233,158,220,164,74,228,63,10,243,109,171,93,139,56,187,111,9,];
    let x1_c1_b2_a0_bytes = [88,170,4,14,40,128,9,37,112,153,51,44,207,24,160,166,202,141,45,176,216,247,252,83,79,125,219,52,45,47,195,0,109,64,17,233,109,171,86,64,101,17,110,125,8,209,220,14,];
    let x1_c1_b2_a1_bytes = [45,80,195,74,220,212,197,127,138,75,183,100,244,133,63,126,203,191,237,238,226,187,191,134,30,11,201,89,71,197,47,97,183,210,75,121,252,204,21,52,14,136,175,8,7,47,128,23,];
    let x2_c0_b0_a0_bytes = [207,123,59,41,71,199,157,119,27,229,2,72,223,13,8,251,25,170,95,253,134,7,18,186,10,80,155,26,153,51,34,22,214,110,93,76,148,141,134,57,169,164,200,199,107,197,75,9,];
    let x2_c0_b0_a1_bytes = [68,69,224,93,98,215,204,82,194,61,18,31,46,41,185,144,206,231,31,51,74,116,181,251,234,202,189,95,0,240,212,50,63,191,129,195,175,199,221,38,23,236,65,18,180,53,202,18,];
    let x2_c0_b1_a0_bytes = [186,144,32,185,138,99,144,82,150,0,247,3,0,51,10,166,185,154,142,185,163,124,12,5,241,57,101,249,220,151,30,50,24,22,21,118,111,53,202,5,18,163,236,234,230,225,225,6,];
    let x2_c0_b1_a1_bytes = [129,230,32,169,143,192,38,68,3,67,71,66,246,63,61,99,79,114,226,254,242,101,101,216,185,46,207,94,64,31,52,228,74,72,187,36,88,116,105,83,26,228,68,174,245,212,238,2,];
    let x2_c0_b2_a0_bytes = [235,52,202,255,51,211,85,55,201,206,154,2,143,64,110,181,182,241,133,199,114,238,98,106,205,96,163,188,29,95,1,45,211,112,47,18,58,43,188,100,77,15,120,149,95,39,65,9,];
    let x2_c0_b2_a1_bytes = [60,162,193,242,39,0,247,34,1,223,4,60,36,176,54,21,250,134,236,109,178,151,83,69,214,150,24,216,254,14,116,40,28,249,139,244,17,29,161,19,15,212,197,187,59,112,115,7,];
    let x2_c1_b0_a0_bytes = [113,218,47,127,22,250,161,186,250,57,145,231,193,242,195,30,103,173,138,96,253,254,214,30,118,142,86,234,124,42,254,83,97,71,171,166,43,92,68,22,97,207,47,60,24,134,121,24,];
    let x2_c1_b0_a1_bytes = [84,142,210,101,68,101,232,128,181,72,18,139,50,49,154,240,63,62,97,119,211,173,237,246,74,158,115,193,34,110,250,10,220,176,199,155,92,240,131,185,207,226,185,61,202,228,92,15,];
    let x2_c1_b1_a0_bytes = [250,135,154,222,108,42,102,23,74,68,83,86,167,98,138,104,253,49,71,167,196,170,135,50,63,98,59,111,134,5,72,17,172,15,206,58,202,72,63,97,246,198,38,226,181,245,80,12,];
    let x2_c1_b1_a1_bytes = [208,204,154,145,76,144,241,217,84,190,117,196,80,78,182,245,92,80,237,39,7,154,54,244,50,180,151,14,148,175,244,47,168,171,100,32,206,241,202,78,149,242,234,181,52,30,223,7,];
    let x2_c1_b2_a0_bytes = [164,221,47,104,91,235,36,227,161,228,125,132,27,80,59,1,226,21,221,186,69,29,202,132,31,136,169,58,117,66,181,20,171,155,111,193,44,170,84,106,66,239,129,183,90,201,16,2,];
    let x2_c1_b2_a1_bytes = [176,82,215,189,194,60,100,195,112,118,145,104,52,208,24,195,55,9,1,239,70,109,90,4,173,106,138,93,2,137,66,97,233,69,145,134,59,136,104,195,16,195,104,13,72,127,208,15,];

   for i in 0..48 {
        assignment.x[0][0][0][i] = M31::from(x0_c0_b0_a0_bytes[i]);
        assignment.x[0][0][1][i] = M31::from(x0_c0_b0_a1_bytes[i]);
        assignment.x[0][1][0][i] = M31::from(x0_c0_b1_a0_bytes[i]);
        assignment.x[0][1][1][i] = M31::from(x0_c0_b1_a1_bytes[i]);
        assignment.x[0][2][0][i] = M31::from(x0_c0_b2_a0_bytes[i]);
        assignment.x[0][2][1][i] = M31::from(x0_c0_b2_a1_bytes[i]);
        assignment.x[1][0][0][i] = M31::from(x0_c1_b0_a0_bytes[i]);
        assignment.x[1][0][1][i] = M31::from(x0_c1_b0_a1_bytes[i]);
        assignment.x[1][1][0][i] = M31::from(x0_c1_b1_a0_bytes[i]);
        assignment.x[1][1][1][i] = M31::from(x0_c1_b1_a1_bytes[i]);
        assignment.x[1][2][0][i] = M31::from(x0_c1_b2_a0_bytes[i]);
        assignment.x[1][2][1][i] = M31::from(x0_c1_b2_a1_bytes[i]);
        assignment.y[0][0][0][i] = M31::from(x1_c0_b0_a0_bytes[i]);
        assignment.y[0][0][1][i] = M31::from(x1_c0_b0_a1_bytes[i]);
        assignment.y[0][1][0][i] = M31::from(x1_c0_b1_a0_bytes[i]);
        assignment.y[0][1][1][i] = M31::from(x1_c0_b1_a1_bytes[i]);
        assignment.y[0][2][0][i] = M31::from(x1_c0_b2_a0_bytes[i]);
        assignment.y[0][2][1][i] = M31::from(x1_c0_b2_a1_bytes[i]);
        assignment.y[1][0][0][i] = M31::from(x1_c1_b0_a0_bytes[i]);
        assignment.y[1][0][1][i] = M31::from(x1_c1_b0_a1_bytes[i]);
        assignment.y[1][1][0][i] = M31::from(x1_c1_b1_a0_bytes[i]);
        assignment.y[1][1][1][i] = M31::from(x1_c1_b1_a1_bytes[i]);
        assignment.y[1][2][0][i] = M31::from(x1_c1_b2_a0_bytes[i]);
        assignment.y[1][2][1][i] = M31::from(x1_c1_b2_a1_bytes[i]);
        assignment.z[0][0][0][i] = M31::from(x2_c0_b0_a0_bytes[i]);
        assignment.z[0][0][1][i] = M31::from(x2_c0_b0_a1_bytes[i]);
        assignment.z[0][1][0][i] = M31::from(x2_c0_b1_a0_bytes[i]);
        assignment.z[0][1][1][i] = M31::from(x2_c0_b1_a1_bytes[i]);
        assignment.z[0][2][0][i] = M31::from(x2_c0_b2_a0_bytes[i]);
        assignment.z[0][2][1][i] = M31::from(x2_c0_b2_a1_bytes[i]);
        assignment.z[1][0][0][i] = M31::from(x2_c1_b0_a0_bytes[i]);
        assignment.z[1][0][1][i] = M31::from(x2_c1_b0_a1_bytes[i]);
        assignment.z[1][1][0][i] = M31::from(x2_c1_b1_a0_bytes[i]);
        assignment.z[1][1][1][i] = M31::from(x2_c1_b1_a1_bytes[i]);
        assignment.z[1][2][0][i] = M31::from(x2_c1_b2_a0_bytes[i]);
        assignment.z[1][2][1][i] = M31::from(x2_c1_b2_a1_bytes[i]);
    }
    
    debug_eval(
        &E12AddCircuit::default(),
        &assignment,
        hint_registry,
    );
}
declare_circuit!(E12SubCircuit {
    a: [[[[Variable; 48]; 2]; 3]; 2],
    b: [[[[Variable; 48]; 2]; 3]; 2],
    c: [[[[Variable; 48]; 2]; 3]; 2],
});

impl GenericDefine<M31Config> for E12SubCircuit<Variable> {
    fn define<Builder: RootAPI<M31Config>>(&self, builder: &mut Builder) {
        let mut ext12 = Ext12::new(builder);

        let a_e12 = GE12 {
            c0: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.a[0][0][0].to_vec(), 0),
                    a1: new_internal_element(self.a[0][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.a[0][1][0].to_vec(), 0),
                    a1: new_internal_element(self.a[0][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.a[0][2][0].to_vec(), 0),
                    a1: new_internal_element(self.a[0][2][1].to_vec(), 0),
                },
            },
            c1: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.a[1][0][0].to_vec(), 0),
                    a1: new_internal_element(self.a[1][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.a[1][1][0].to_vec(), 0),
                    a1: new_internal_element(self.a[1][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.a[1][2][0].to_vec(), 0),
                    a1: new_internal_element(self.a[1][2][1].to_vec(), 0),
                },
            },
        };

        let b_e12 = GE12 {
            c0: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.b[0][0][0].to_vec(), 0),
                    a1: new_internal_element(self.b[0][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.b[0][1][0].to_vec(), 0),
                    a1: new_internal_element(self.b[0][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.b[0][2][0].to_vec(), 0),
                    a1: new_internal_element(self.b[0][2][1].to_vec(), 0),
                },
            },
            c1: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.b[1][0][0].to_vec(), 0),
                    a1: new_internal_element(self.b[1][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.b[1][1][0].to_vec(), 0),
                    a1: new_internal_element(self.b[1][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.b[1][2][0].to_vec(), 0),
                    a1: new_internal_element(self.b[1][2][1].to_vec(), 0),
                },
            },
        };

        let c_e12 = GE12 {
            c0: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.c[0][0][0].to_vec(), 0),
                    a1: new_internal_element(self.c[0][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.c[0][1][0].to_vec(), 0),
                    a1: new_internal_element(self.c[0][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.c[0][2][0].to_vec(), 0),
                    a1: new_internal_element(self.c[0][2][1].to_vec(), 0),
                },
            },
            c1: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.c[1][0][0].to_vec(), 0),
                    a1: new_internal_element(self.c[1][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.c[1][1][0].to_vec(), 0),
                    a1: new_internal_element(self.c[1][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.c[1][2][0].to_vec(), 0),
                    a1: new_internal_element(self.c[1][2][1].to_vec(), 0),
                },
            },
        };

        let z = ext12.sub(builder, &a_e12, &b_e12);
        ext12.assert_isequal(builder, &z, &c_e12);
        ext12.ext6.ext2.fp.check_mul(builder);
        ext12.ext6.ext2.fp.table.final_check(builder);
    }
}

#[test]
fn test_e12_sub() {
    let compile_result =
        compile_generic(&E12SubCircuit::default(), CompileOptions::default()).unwrap();
    let mut hint_registry = HintRegistry::<M31>::new();
    register_hint(&mut hint_registry);

    let mut assignment = E12SubCircuit::<M31> {
        a: [[[[M31::from(0); 48]; 2]; 3]; 2],
        b: [[[[M31::from(0); 48]; 2]; 3]; 2],
        c: [[[[M31::from(0); 48]; 2]; 3]; 2],
    };

    let x0_c0_b0_a0_bytes = [197,236,193,85,161,111,30,106,84,151,195,17,249,224,84,244,234,151,155,63,74,153,175,165,235,125,153,130,243,107,14,105,245,28,233,106,75,57,94,106,84,180,23,57,67,110,184,10,];
let x0_c0_b0_a1_bytes = [59,96,28,50,133,228,182,73,66,218,225,164,193,187,245,231,228,192,66,73,171,154,154,62,133,130,233,245,172,151,229,221,180,146,34,210,144,85,244,82,184,183,27,180,223,136,102,24,];
let x0_c0_b1_a0_bytes = [84,55,219,118,151,133,30,81,23,129,216,253,231,146,81,239,82,143,143,240,153,190,91,53,196,35,118,126,126,117,228,158,50,171,35,147,148,104,198,50,111,65,153,100,245,126,124,7,];
let x0_c0_b1_a1_bytes = [158,71,191,118,128,142,50,104,161,113,119,153,140,128,153,6,169,32,115,6,250,209,208,97,194,1,162,91,12,42,22,245,136,71,91,95,227,52,40,208,108,112,216,18,58,137,192,1,];
let x0_c0_b2_a0_bytes = [228,37,132,99,194,152,42,52,22,111,105,49,77,137,143,217,244,72,169,243,233,48,144,134,104,208,140,34,253,229,139,181,9,39,20,5,49,42,213,22,78,66,164,172,111,223,186,22,];
let x0_c0_b2_a1_bytes = [91,255,84,235,130,162,183,217,231,118,130,247,180,1,189,144,216,166,141,55,72,168,144,255,240,224,253,181,195,202,154,136,143,131,24,12,18,54,102,200,132,179,33,73,73,129,120,24,];
let x0_c1_b0_a0_bytes = [75,145,107,24,225,40,95,38,248,143,36,81,242,205,106,97,93,79,202,24,215,215,203,153,98,58,232,124,142,40,126,86,171,9,120,56,12,102,208,245,103,47,55,136,96,157,196,1,];
let x0_c1_b0_a1_bytes = [195,95,9,22,123,87,85,52,125,17,135,205,148,125,41,154,196,207,18,95,210,76,5,80,165,167,180,14,149,98,136,29,247,65,214,62,90,127,47,44,19,47,16,84,210,45,33,3,];
let x0_c1_b1_a0_bytes = [67,64,200,83,56,98,37,156,128,197,145,165,24,7,119,161,36,53,81,104,132,26,28,154,249,99,147,13,200,123,226,105,94,31,96,107,114,36,246,164,198,23,239,186,38,4,150,11,];
let x0_c1_b1_a1_bytes = [120,243,96,185,212,141,147,104,52,239,147,173,134,47,255,170,192,225,233,197,7,190,254,207,196,69,228,67,11,209,193,162,29,33,62,134,198,93,171,104,36,55,224,195,116,124,37,5,];
let x0_c1_b2_a0_bytes = [149,80,66,197,78,71,174,41,148,153,187,24,17,86,155,33,16,86,221,137,135,115,244,2,255,150,239,226,231,115,224,37,155,126,196,79,207,144,253,16,159,113,37,120,77,255,73,22,];
let x0_c1_b2_a1_bytes = [190,175,107,235,207,189,162,102,173,62,208,181,179,166,36,90,114,111,210,198,113,141,199,109,94,157,183,9,128,240,121,117,148,236,238,69,107,66,217,41,236,99,80,244,190,82,151,1,];
let x1_c0_b0_a0_bytes = [232,141,62,55,243,245,168,210,31,237,239,153,14,209,115,1,206,147,183,64,152,81,49,18,190,179,192,37,84,115,137,165,244,132,222,69,0,30,137,145,103,129,61,52,250,155,219,4,];
let x1_c0_b0_a1_bytes = [113,139,120,115,225,148,22,187,109,115,126,91,111,145,171,208,110,106,149,194,93,202,135,38,207,224,84,228,29,20,108,242,236,97,233,108,121,144,23,153,40,223,98,234,188,44,242,6,];
let x1_c0_b1_a0_bytes = [152,83,177,81,25,169,168,112,215,237,121,175,120,129,75,46,55,200,16,106,154,231,73,168,62,216,151,228,249,41,11,107,158,140,67,215,117,16,84,45,234,74,151,254,184,219,116,0,];
let x1_c0_b1_a1_bytes = [68,35,46,47,154,117,41,42,243,148,223,144,111,107,140,207,164,68,84,243,64,128,254,216,177,233,131,227,40,19,194,153,248,80,201,0,127,63,59,155,222,127,81,60,26,190,33,15,];
let x1_c0_b2_a0_bytes = [248,133,135,6,150,86,28,203,165,53,190,226,99,10,36,47,226,178,239,209,159,91,220,5,67,62,117,35,108,130,199,12,45,245,84,40,110,201,159,184,237,175,154,239,164,187,131,1,];
let x1_c0_b2_a1_bytes = [68,107,158,70,92,137,135,220,212,245,24,214,217,210,137,220,42,191,194,42,243,143,219,231,52,64,89,157,205,97,52,209,9,61,136,37,202,247,64,166,163,249,26,95,59,255,237,7,];
let x1_c1_b0_a0_bytes = [169,12,166,142,127,221,90,52,130,240,103,229,157,212,117,57,95,237,195,145,196,87,41,204,201,55,101,137,193,53,23,73,177,252,212,131,1,89,170,171,222,181,216,219,162,41,228,8,];
let x1_c1_b0_a1_bytes = [237,98,101,211,49,237,157,16,6,61,83,201,3,96,185,153,250,216,184,117,159,246,233,96,23,119,118,103,88,80,126,68,66,214,147,46,209,159,243,75,204,240,192,84,231,18,57,17,];
let x1_c1_b1_a0_bytes = [104,144,181,81,179,227,108,37,237,241,87,182,122,63,188,228,195,34,131,244,136,121,187,97,57,55,255,12,229,30,113,5,129,97,18,46,21,43,137,24,204,21,47,114,88,123,199,9,];
let x1_c1_b1_a1_bytes = [219,73,222,238,62,66,133,212,134,204,165,110,75,169,34,254,78,131,51,67,27,193,8,56,180,137,126,251,241,176,69,38,15,118,107,98,68,68,96,1,144,214,29,31,83,179,138,6,];
let x1_c1_b2_a0_bytes = [200,135,142,179,186,161,77,83,223,201,62,131,26,198,122,50,188,167,41,219,122,80,74,9,1,233,94,222,127,179,185,37,73,200,87,78,147,149,225,52,187,134,144,110,101,198,248,11,];
let x1_c1_b2_a1_bytes = [161,101,7,76,21,58,5,167,239,173,64,201,247,135,227,46,142,173,1,178,43,222,120,104,27,246,152,18,240,122,233,85,242,136,136,113,15,145,142,200,124,118,22,138,12,152,9,22,];
let x2_c0_b0_a0_bytes = [221,94,131,30,174,121,117,151,52,170,211,119,234,15,225,242,28,4,228,254,177,71,126,147,45,202,216,92,159,248,132,195,0,152,10,37,75,27,213,216,236,50,218,4,73,210,220,5,];
let x2_c0_b0_a1_bytes = [202,212,163,190,163,79,160,142,212,102,99,73,82,42,74,23,118,86,173,134,77,208,18,24,182,161,148,17,143,131,121,235,199,48,57,101,23,197,220,185,143,216,184,201,34,92,116,17,];
let x2_c0_b1_a0_bytes = [188,227,41,37,126,220,117,224,63,147,94,78,111,17,6,193,27,199,126,134,255,214,17,141,133,75,222,153,132,75,217,51,148,30,224,187,30,88,114,5,133,246,1,102,60,163,7,7,];
let x2_c0_b1_a1_bytes = [5,207,144,71,230,24,8,248,173,220,235,185,27,21,185,85,40,210,207,9,90,36,3,240,207,42,163,107,104,98,203,191,103,163,221,161,26,157,8,128,40,215,6,16,10,221,159,12,];
let x2_c0_b2_a0_bytes = [236,159,252,92,44,66,14,105,112,57,171,78,233,126,107,170,18,150,185,33,74,213,179,128,37,146,23,255,144,99,196,168,220,49,191,220,194,96,53,94,96,146,9,189,202,35,55,21,];
let x2_c0_b2_a1_bytes = [23,148,182,164,38,25,48,253,18,129,105,33,219,46,51,180,173,231,202,12,85,24,181,23,188,160,164,24,246,104,102,183,133,70,144,230,71,62,37,34,225,185,6,234,13,130,138,16,];
let x2_c1_b0_a0_bytes = [77,47,197,137,97,75,3,172,117,159,16,29,83,249,160,70,34,88,183,125,179,82,211,52,88,21,8,231,81,62,222,113,209,185,238,247,192,180,65,149,35,96,222,229,167,133,225,18,];
let x2_c1_b0_a1_bytes = [129,167,163,66,73,106,182,221,118,212,135,181,143,29,28,31,238,236,10,224,211,40,76,86,77,67,195,154,193,93,129,61,140,24,142,83,63,135,87,43,225,36,207,56,213,44,233,11,];
let x2_c1_b1_a0_bytes = [219,175,18,2,133,126,184,118,147,211,57,239,157,199,186,188,96,18,206,115,251,160,96,56,192,44,148,0,227,92,113,100,221,189,77,61,93,249,108,140,250,1,192,72,206,136,206,1,];
let x2_c1_b1_a1_bytes = [72,84,130,202,149,75,13,78,173,34,66,240,57,134,136,203,149,84,103,121,141,207,38,255,207,206,234,59,158,107,243,224,229,87,30,103,56,193,102,178,46,71,66,222,11,219,155,24,];
let x2_c1_b2_a0_bytes = [205,200,179,17,148,165,96,214,180,207,124,149,246,143,32,239,83,174,179,174,12,35,170,249,253,173,144,4,104,192,38,0,82,182,108,1,60,251,27,220,227,234,148,9,232,56,81,10,];
let x2_c1_b2_a1_bytes = [200,244,99,159,186,131,156,121,189,144,227,157,186,30,237,73,8,184,129,11,231,129,127,108,2,186,163,234,20,193,7,132,121,16,178,23,18,89,102,172,9,212,185,163,156,204,142,5,];

    for i in 0..48 {
        assignment.a[0][0][0][i] = M31::from(x0_c0_b0_a0_bytes[i]);
        assignment.a[0][0][1][i] = M31::from(x0_c0_b0_a1_bytes[i]);
        assignment.a[0][1][0][i] = M31::from(x0_c0_b1_a0_bytes[i]);
        assignment.a[0][1][1][i] = M31::from(x0_c0_b1_a1_bytes[i]);
        assignment.a[0][2][0][i] = M31::from(x0_c0_b2_a0_bytes[i]);
        assignment.a[0][2][1][i] = M31::from(x0_c0_b2_a1_bytes[i]);
        assignment.a[1][0][0][i] = M31::from(x0_c1_b0_a0_bytes[i]);
        assignment.a[1][0][1][i] = M31::from(x0_c1_b0_a1_bytes[i]);
        assignment.a[1][1][0][i] = M31::from(x0_c1_b1_a0_bytes[i]);
        assignment.a[1][1][1][i] = M31::from(x0_c1_b1_a1_bytes[i]);
        assignment.a[1][2][0][i] = M31::from(x0_c1_b2_a0_bytes[i]);
        assignment.a[1][2][1][i] = M31::from(x0_c1_b2_a1_bytes[i]);
        assignment.b[0][0][0][i] = M31::from(x1_c0_b0_a0_bytes[i]);
        assignment.b[0][0][1][i] = M31::from(x1_c0_b0_a1_bytes[i]);
        assignment.b[0][1][0][i] = M31::from(x1_c0_b1_a0_bytes[i]);
        assignment.b[0][1][1][i] = M31::from(x1_c0_b1_a1_bytes[i]);
        assignment.b[0][2][0][i] = M31::from(x1_c0_b2_a0_bytes[i]);
        assignment.b[0][2][1][i] = M31::from(x1_c0_b2_a1_bytes[i]);
        assignment.b[1][0][0][i] = M31::from(x1_c1_b0_a0_bytes[i]);
        assignment.b[1][0][1][i] = M31::from(x1_c1_b0_a1_bytes[i]);
        assignment.b[1][1][0][i] = M31::from(x1_c1_b1_a0_bytes[i]);
        assignment.b[1][1][1][i] = M31::from(x1_c1_b1_a1_bytes[i]);
        assignment.b[1][2][0][i] = M31::from(x1_c1_b2_a0_bytes[i]);
        assignment.b[1][2][1][i] = M31::from(x1_c1_b2_a1_bytes[i]);
        assignment.c[0][0][0][i] = M31::from(x2_c0_b0_a0_bytes[i]);
        assignment.c[0][0][1][i] = M31::from(x2_c0_b0_a1_bytes[i]);
        assignment.c[0][1][0][i] = M31::from(x2_c0_b1_a0_bytes[i]);
        assignment.c[0][1][1][i] = M31::from(x2_c0_b1_a1_bytes[i]);
        assignment.c[0][2][0][i] = M31::from(x2_c0_b2_a0_bytes[i]);
        assignment.c[0][2][1][i] = M31::from(x2_c0_b2_a1_bytes[i]);
        assignment.c[1][0][0][i] = M31::from(x2_c1_b0_a0_bytes[i]);
        assignment.c[1][0][1][i] = M31::from(x2_c1_b0_a1_bytes[i]);
        assignment.c[1][1][0][i] = M31::from(x2_c1_b1_a0_bytes[i]);
        assignment.c[1][1][1][i] = M31::from(x2_c1_b1_a1_bytes[i]);
        assignment.c[1][2][0][i] = M31::from(x2_c1_b2_a0_bytes[i]);
        assignment.c[1][2][1][i] = M31::from(x2_c1_b2_a1_bytes[i]);
    }

    debug_eval(&E12SubCircuit::default(), &assignment, hint_registry);
}


declare_circuit!(E12MulCircuit {
    a: [[[[Variable; 48]; 2]; 3]; 2],
    b: [[[[Variable; 48]; 2]; 3]; 2],
    c: [[[[Variable; 48]; 2]; 3]; 2],
});

impl GenericDefine<M31Config> for E12MulCircuit<Variable> {
    fn define<Builder: RootAPI<M31Config>>(&self, builder: &mut Builder) {
        let mut ext12 = Ext12::new(builder);

        let a_e12 = GE12 {
            c0: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.a[0][0][0].to_vec(), 0),
                    a1: new_internal_element(self.a[0][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.a[0][1][0].to_vec(), 0),
                    a1: new_internal_element(self.a[0][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.a[0][2][0].to_vec(), 0),
                    a1: new_internal_element(self.a[0][2][1].to_vec(), 0),
                },
            },
            c1: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.a[1][0][0].to_vec(), 0),
                    a1: new_internal_element(self.a[1][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.a[1][1][0].to_vec(), 0),
                    a1: new_internal_element(self.a[1][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.a[1][2][0].to_vec(), 0),
                    a1: new_internal_element(self.a[1][2][1].to_vec(), 0),
                },
            },
        };

        let b_e12 = GE12 {
            c0: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.b[0][0][0].to_vec(), 0),
                    a1: new_internal_element(self.b[0][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.b[0][1][0].to_vec(), 0),
                    a1: new_internal_element(self.b[0][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.b[0][2][0].to_vec(), 0),
                    a1: new_internal_element(self.b[0][2][1].to_vec(), 0),
                },
            },
            c1: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.b[1][0][0].to_vec(), 0),
                    a1: new_internal_element(self.b[1][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.b[1][1][0].to_vec(), 0),
                    a1: new_internal_element(self.b[1][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.b[1][2][0].to_vec(), 0),
                    a1: new_internal_element(self.b[1][2][1].to_vec(), 0),
                },
            },
        };

        let c_e12 = GE12 {
            c0: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.c[0][0][0].to_vec(), 0),
                    a1: new_internal_element(self.c[0][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.c[0][1][0].to_vec(), 0),
                    a1: new_internal_element(self.c[0][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.c[0][2][0].to_vec(), 0),
                    a1: new_internal_element(self.c[0][2][1].to_vec(), 0),
                },
            },
            c1: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.c[1][0][0].to_vec(), 0),
                    a1: new_internal_element(self.c[1][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.c[1][1][0].to_vec(), 0),
                    a1: new_internal_element(self.c[1][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.c[1][2][0].to_vec(), 0),
                    a1: new_internal_element(self.c[1][2][1].to_vec(), 0),
                },
            },
        };

        let z = ext12.mul(builder, &a_e12, &b_e12);
        ext12.assert_isequal(builder, &z, &c_e12);
        ext12.ext6.ext2.fp.check_mul(builder);
        ext12.ext6.ext2.fp.table.final_check(builder);
    }
}

#[test]
fn test_e12_mul() {
    let compile_result =
        compile_generic(&E12MulCircuit::default(), CompileOptions::default()).unwrap();
    let mut hint_registry = HintRegistry::<M31>::new();
    register_hint(&mut hint_registry);

    let mut assignment = E12MulCircuit::<M31> {
        a: [[[[M31::from(0); 48]; 2]; 3]; 2],
        b: [[[[M31::from(0); 48]; 2]; 3]; 2],
        c: [[[[M31::from(0); 48]; 2]; 3]; 2],
    };

    let x0_c0_b0_a0_bytes = [18,16,175,85,34,237,118,71,162,164,89,178,78,181,29,51,79,100,35,97,196,220,121,215,157,189,144,26,67,25,143,143,42,101,231,240,230,220,139,229,187,86,239,244,109,91,143,20,];
    let x0_c0_b0_a1_bytes = [104,153,197,146,135,101,130,39,74,182,160,38,197,224,5,133,142,105,202,217,215,240,244,171,157,55,89,59,188,205,135,43,127,31,166,190,9,193,93,205,58,226,101,14,153,21,234,22,];
    let x0_c0_b1_a0_bytes = [126,212,100,36,202,52,184,67,214,199,123,245,2,167,137,57,81,54,78,8,204,178,55,15,220,40,57,37,167,232,27,33,243,213,212,233,46,43,145,49,208,94,159,54,61,86,74,22,];
    let x0_c0_b1_a1_bytes = [174,111,11,165,30,60,48,155,87,253,31,26,63,238,208,50,127,61,238,214,152,200,10,111,92,23,141,127,190,250,186,237,78,143,238,113,111,124,32,10,61,131,95,58,154,188,144,25,];
    let x0_c0_b2_a0_bytes = [59,200,148,183,6,226,234,205,189,41,155,50,205,1,73,159,234,93,20,65,7,210,176,195,242,149,31,36,66,79,103,232,182,29,129,100,127,55,143,74,76,224,7,87,128,229,156,13,];
    let x0_c0_b2_a1_bytes = [110,72,137,164,201,4,40,254,210,231,146,39,192,152,171,24,237,83,153,179,26,97,200,122,36,82,239,217,181,231,62,128,66,227,0,198,91,252,165,196,81,198,154,73,96,55,209,19,];
    let x0_c1_b0_a0_bytes = [169,129,186,227,169,163,212,206,238,76,175,179,26,251,188,55,225,254,135,143,106,185,34,137,192,89,157,244,186,116,163,155,250,100,254,217,201,88,143,57,13,253,249,223,180,181,154,1,];
    let x0_c1_b0_a1_bytes = [241,145,54,93,184,84,47,57,100,101,64,216,140,119,185,24,79,78,187,112,137,186,170,29,142,240,58,182,135,206,87,185,164,140,72,144,75,219,55,197,124,20,45,213,71,6,195,7,];
    let x0_c1_b1_a0_bytes = [205,64,90,100,21,169,136,39,56,72,95,160,189,175,183,219,70,48,253,114,208,195,195,42,203,148,99,109,232,156,175,222,224,133,192,52,178,135,98,208,120,253,167,40,242,93,35,25,];
    let x0_c1_b1_a1_bytes = [3,148,43,205,241,107,73,27,92,128,127,56,26,71,93,197,106,244,30,151,227,100,3,100,35,57,155,142,253,223,146,199,123,9,30,111,201,199,61,77,22,183,200,140,225,254,194,20,];
    let x0_c1_b2_a0_bytes = [50,105,205,33,216,5,48,84,66,141,202,6,27,142,141,74,204,171,60,145,125,247,88,64,93,126,118,112,109,230,100,16,42,239,204,160,230,2,7,85,120,155,87,196,244,159,199,20,];
    let x0_c1_b2_a1_bytes = [11,173,240,71,15,10,199,212,101,196,123,200,143,223,216,254,40,78,66,163,117,205,134,253,18,21,17,37,196,124,210,118,177,48,105,105,114,222,224,205,37,180,65,198,34,48,34,19,];
    let x1_c0_b0_a0_bytes = [240,137,36,51,174,210,159,102,67,7,163,220,57,196,207,116,18,202,148,248,6,45,135,188,79,72,55,149,74,111,220,241,23,21,151,196,186,87,250,144,144,213,24,190,214,125,110,0,];
    let x1_c0_b0_a1_bytes = [60,27,22,130,117,251,130,122,140,235,142,212,10,48,246,0,141,46,146,86,0,78,161,219,203,39,120,253,162,34,241,239,135,28,181,205,147,187,157,15,119,201,81,87,222,90,58,15,];
    let x1_c0_b1_a0_bytes = [73,70,72,123,87,235,173,13,165,233,46,210,182,119,13,209,194,46,94,218,156,61,214,26,55,96,204,141,85,154,101,53,136,157,105,5,166,92,37,60,137,148,88,87,165,203,87,7,];
    let x1_c0_b1_a1_bytes = [251,149,3,244,35,194,49,215,250,29,193,89,177,75,111,95,111,154,179,253,102,196,56,147,204,115,142,158,81,35,6,136,144,196,124,75,34,79,141,40,83,27,86,225,184,50,232,8,];
    let x1_c0_b2_a0_bytes = [234,29,186,114,252,192,80,101,188,72,170,15,249,50,15,0,160,97,98,53,174,3,132,228,15,4,19,169,15,44,22,142,62,56,151,39,209,206,103,243,213,24,22,195,30,64,99,17,];
    let x1_c0_b2_a1_bytes = [41,14,48,194,233,49,189,213,184,242,130,15,112,59,59,234,226,157,204,127,56,179,33,102,35,151,38,172,186,116,139,125,145,252,155,113,15,235,96,231,238,29,176,208,83,108,34,2,];
    let x1_c1_b0_a0_bytes = [217,237,38,213,242,122,12,249,193,156,147,167,44,167,3,183,85,155,233,78,216,78,93,112,51,27,189,239,13,26,99,243,161,105,227,210,70,112,48,163,95,44,166,114,32,48,105,5,];
    let x1_c1_b0_a1_bytes = [191,202,154,207,61,76,176,195,236,143,41,42,233,188,57,152,85,0,209,84,229,123,83,90,140,34,165,96,229,100,135,105,223,248,110,29,49,133,47,184,223,49,107,242,204,125,92,3,];
    let x1_c1_b1_a0_bytes = [222,196,209,22,166,64,174,112,126,200,126,250,49,210,117,146,45,137,127,17,219,141,59,149,231,145,239,87,50,126,73,225,42,34,121,105,159,119,218,242,58,177,63,23,17,41,141,8,];
    let x1_c1_b1_a1_bytes = [51,253,245,231,88,162,251,225,148,169,24,17,157,53,128,177,87,114,85,154,248,125,173,180,139,181,126,221,114,103,18,252,227,219,115,161,71,38,91,200,247,35,62,25,118,250,65,0,];
    let x1_c1_b2_a0_bytes = [60,154,232,54,209,216,161,46,119,93,48,165,158,118,33,17,110,132,136,27,135,15,232,41,84,241,133,44,214,113,211,204,78,161,220,224,59,249,51,242,55,121,161,124,16,252,218,12,];
    let x1_c1_b2_a1_bytes = [137,242,221,198,166,207,120,212,128,29,46,23,109,110,227,228,253,14,75,143,148,245,84,86,227,73,113,139,53,141,58,222,227,204,186,104,124,18,92,243,14,223,234,223,53,146,68,22,];
    let x2_c0_b0_a0_bytes = [1,149,245,118,70,112,151,116,114,158,58,126,125,134,169,173,222,62,254,247,138,110,222,181,49,16,20,74,190,252,59,26,36,244,53,89,3,29,193,41,53,209,151,162,227,23,35,0,];
    let x2_c0_b0_a1_bytes = [198,137,108,161,94,178,221,160,92,142,20,161,203,198,212,161,200,102,184,1,149,19,54,172,181,0,3,60,164,25,179,27,126,101,101,152,48,39,140,137,227,188,234,142,37,82,42,4,];
    let x2_c0_b1_a0_bytes = [214,32,230,177,23,76,224,158,211,4,191,255,210,124,182,226,204,174,70,49,245,52,187,68,199,33,75,141,112,46,163,151,1,33,37,156,0,98,15,207,86,18,181,185,56,135,13,21,];
    let x2_c0_b1_a1_bytes = [237,204,148,175,56,19,91,99,62,247,203,193,89,176,166,172,184,135,23,202,116,113,247,209,30,200,205,54,205,157,22,248,203,154,207,92,217,65,253,33,229,230,110,97,247,33,227,2,];
    let x2_c0_b2_a0_bytes = [152,32,127,72,230,253,163,95,208,104,71,35,71,74,212,182,56,212,49,178,60,242,97,255,142,26,231,104,20,239,71,46,18,172,158,162,119,39,155,4,115,149,45,17,160,11,183,23,];
    let x2_c0_b2_a1_bytes = [214,55,28,255,211,238,206,210,80,24,120,165,76,1,7,137,190,11,229,167,236,55,145,134,15,8,208,168,180,16,172,229,206,73,58,192,98,16,104,193,130,66,39,57,178,252,154,5,];
    let x2_c1_b0_a0_bytes = [19,208,0,191,6,160,11,114,241,154,85,194,234,149,134,185,117,13,200,110,62,249,86,202,195,194,53,143,244,54,68,254,65,245,221,102,189,221,246,48,202,113,195,17,47,172,205,16,];
    let x2_c1_b0_a1_bytes = [24,133,121,38,233,140,70,206,19,114,131,40,250,61,165,157,3,218,12,156,3,36,100,173,78,73,161,18,88,169,101,4,224,138,37,192,33,69,119,196,203,122,166,212,20,40,199,18,];
    let x2_c1_b1_a0_bytes = [58,180,157,138,178,143,59,160,99,147,56,53,155,35,65,227,23,162,191,243,139,206,20,109,42,13,184,41,77,101,92,30,49,177,61,60,171,10,114,10,185,131,252,40,88,232,201,10,];
    let x2_c1_b1_a1_bytes = [117,238,170,146,84,80,82,70,144,134,148,70,182,153,18,73,252,151,171,118,161,113,93,115,101,127,97,90,146,232,114,159,164,237,232,31,140,217,160,112,142,153,50,230,151,207,201,7,];
    let x2_c1_b2_a0_bytes = [218,19,179,196,132,93,249,221,47,165,80,237,178,80,214,236,26,67,226,252,234,204,11,109,4,246,171,23,82,14,26,104,36,222,236,91,194,103,215,93,97,69,49,212,61,2,222,11,];
    let x2_c1_b2_a1_bytes = [8,132,51,137,1,206,121,67,104,212,9,238,140,14,73,74,65,177,167,226,127,90,220,71,34,121,96,219,11,245,16,53,63,140,54,254,35,201,17,108,96,16,132,144,60,143,127,3,];

    for i in 0..48 {
        assignment.a[0][0][0][i] = M31::from(x0_c0_b0_a0_bytes[i]);
        assignment.a[0][0][1][i] = M31::from(x0_c0_b0_a1_bytes[i]);
        assignment.a[0][1][0][i] = M31::from(x0_c0_b1_a0_bytes[i]);
        assignment.a[0][1][1][i] = M31::from(x0_c0_b1_a1_bytes[i]);
        assignment.a[0][2][0][i] = M31::from(x0_c0_b2_a0_bytes[i]);
        assignment.a[0][2][1][i] = M31::from(x0_c0_b2_a1_bytes[i]);
        assignment.a[1][0][0][i] = M31::from(x0_c1_b0_a0_bytes[i]);
        assignment.a[1][0][1][i] = M31::from(x0_c1_b0_a1_bytes[i]);
        assignment.a[1][1][0][i] = M31::from(x0_c1_b1_a0_bytes[i]);
        assignment.a[1][1][1][i] = M31::from(x0_c1_b1_a1_bytes[i]);
        assignment.a[1][2][0][i] = M31::from(x0_c1_b2_a0_bytes[i]);
        assignment.a[1][2][1][i] = M31::from(x0_c1_b2_a1_bytes[i]);
        assignment.b[0][0][0][i] = M31::from(x1_c0_b0_a0_bytes[i]);
        assignment.b[0][0][1][i] = M31::from(x1_c0_b0_a1_bytes[i]);
        assignment.b[0][1][0][i] = M31::from(x1_c0_b1_a0_bytes[i]);
        assignment.b[0][1][1][i] = M31::from(x1_c0_b1_a1_bytes[i]);
        assignment.b[0][2][0][i] = M31::from(x1_c0_b2_a0_bytes[i]);
        assignment.b[0][2][1][i] = M31::from(x1_c0_b2_a1_bytes[i]);
        assignment.b[1][0][0][i] = M31::from(x1_c1_b0_a0_bytes[i]);
        assignment.b[1][0][1][i] = M31::from(x1_c1_b0_a1_bytes[i]);
        assignment.b[1][1][0][i] = M31::from(x1_c1_b1_a0_bytes[i]);
        assignment.b[1][1][1][i] = M31::from(x1_c1_b1_a1_bytes[i]);
        assignment.b[1][2][0][i] = M31::from(x1_c1_b2_a0_bytes[i]);
        assignment.b[1][2][1][i] = M31::from(x1_c1_b2_a1_bytes[i]);
        assignment.c[0][0][0][i] = M31::from(x2_c0_b0_a0_bytes[i]);
        assignment.c[0][0][1][i] = M31::from(x2_c0_b0_a1_bytes[i]);
        assignment.c[0][1][0][i] = M31::from(x2_c0_b1_a0_bytes[i]);
        assignment.c[0][1][1][i] = M31::from(x2_c0_b1_a1_bytes[i]);
        assignment.c[0][2][0][i] = M31::from(x2_c0_b2_a0_bytes[i]);
        assignment.c[0][2][1][i] = M31::from(x2_c0_b2_a1_bytes[i]);
        assignment.c[1][0][0][i] = M31::from(x2_c1_b0_a0_bytes[i]);
        assignment.c[1][0][1][i] = M31::from(x2_c1_b0_a1_bytes[i]);
        assignment.c[1][1][0][i] = M31::from(x2_c1_b1_a0_bytes[i]);
        assignment.c[1][1][1][i] = M31::from(x2_c1_b1_a1_bytes[i]);
        assignment.c[1][2][0][i] = M31::from(x2_c1_b2_a0_bytes[i]);
        assignment.c[1][2][1][i] = M31::from(x2_c1_b2_a1_bytes[i]);
    }
    debug_eval(&E12MulCircuit::default(), &assignment, hint_registry);
}

declare_circuit!(E12DivCircuit {
    a: [[[[Variable; 48]; 2]; 3]; 2],
    b: [[[[Variable; 48]; 2]; 3]; 2],
    c: [[[[Variable; 48]; 2]; 3]; 2],
});

impl GenericDefine<M31Config> for E12DivCircuit<Variable> {
    fn define<Builder: RootAPI<M31Config>>(&self, builder: &mut Builder) {
        let mut ext12 = Ext12::new(builder);

        let a_e12 = GE12 {
            c0: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.a[0][0][0].to_vec(), 0),
                    a1: new_internal_element(self.a[0][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.a[0][1][0].to_vec(), 0),
                    a1: new_internal_element(self.a[0][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.a[0][2][0].to_vec(), 0),
                    a1: new_internal_element(self.a[0][2][1].to_vec(), 0),
                },
            },
            c1: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.a[1][0][0].to_vec(), 0),
                    a1: new_internal_element(self.a[1][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.a[1][1][0].to_vec(), 0),
                    a1: new_internal_element(self.a[1][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.a[1][2][0].to_vec(), 0),
                    a1: new_internal_element(self.a[1][2][1].to_vec(), 0),
                },
            },
        };

        let b_e12 = GE12 {
            c0: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.b[0][0][0].to_vec(), 0),
                    a1: new_internal_element(self.b[0][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.b[0][1][0].to_vec(), 0),
                    a1: new_internal_element(self.b[0][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.b[0][2][0].to_vec(), 0),
                    a1: new_internal_element(self.b[0][2][1].to_vec(), 0),
                },
            },
            c1: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.b[1][0][0].to_vec(), 0),
                    a1: new_internal_element(self.b[1][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.b[1][1][0].to_vec(), 0),
                    a1: new_internal_element(self.b[1][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.b[1][2][0].to_vec(), 0),
                    a1: new_internal_element(self.b[1][2][1].to_vec(), 0),
                },
            },
        };

        let c_e12 = GE12 {
            c0: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.c[0][0][0].to_vec(), 0),
                    a1: new_internal_element(self.c[0][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.c[0][1][0].to_vec(), 0),
                    a1: new_internal_element(self.c[0][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.c[0][2][0].to_vec(), 0),
                    a1: new_internal_element(self.c[0][2][1].to_vec(), 0),
                },
            },
            c1: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.c[1][0][0].to_vec(), 0),
                    a1: new_internal_element(self.c[1][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.c[1][1][0].to_vec(), 0),
                    a1: new_internal_element(self.c[1][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.c[1][2][0].to_vec(), 0),
                    a1: new_internal_element(self.c[1][2][1].to_vec(), 0),
                },
            },
        };

        let z = ext12.div(builder, &a_e12, &b_e12);
        ext12.assert_isequal(builder, &z, &c_e12);
        ext12.ext6.ext2.fp.check_mul(builder);
        ext12.ext6.ext2.fp.table.final_check(builder);
    }
}

#[test]
fn test_e12_div() {
    let compile_result =
        compile_generic(&E12DivCircuit::default(), CompileOptions::default()).unwrap();
    let mut hint_registry = HintRegistry::<M31>::new();
    register_hint(&mut hint_registry);

    let mut assignment = E12DivCircuit::<M31> {
        a: [[[[M31::from(0); 48]; 2]; 3]; 2],
        b: [[[[M31::from(0); 48]; 2]; 3]; 2],
        c: [[[[M31::from(0); 48]; 2]; 3]; 2],
    };

    let x0_c0_b0_a0_bytes = [254,180,220,147,183,118,153,36,195,182,38,75,52,106,65,31,129,247,165,36,249,44,176,1,42,106,237,185,148,192,231,0,123,186,60,239,65,203,166,161,15,211,65,114,65,36,80,3,];
    let x0_c0_b0_a1_bytes = [58,127,245,147,170,27,20,107,100,56,192,22,167,172,88,219,98,126,91,86,29,142,117,156,166,36,223,50,161,179,178,252,125,164,147,159,249,111,70,48,106,58,142,112,204,211,72,18,];
    let x0_c0_b1_a0_bytes = [151,18,147,147,3,131,131,230,185,24,54,136,249,234,141,241,80,44,100,169,203,250,245,208,130,171,36,70,145,68,7,223,110,161,240,4,188,221,252,143,243,16,70,147,121,203,207,23,];
    let x0_c0_b1_a1_bytes = [121,192,157,27,84,232,248,218,216,193,26,58,161,185,51,106,144,142,48,62,254,62,201,224,38,98,44,105,90,96,51,6,219,241,23,198,109,39,66,76,236,6,84,98,197,72,92,7,];
    let x0_c0_b2_a0_bytes = [183,181,165,60,147,229,250,166,11,193,79,192,12,161,71,94,96,212,33,91,80,90,141,52,246,64,44,85,182,252,39,164,76,235,131,247,38,57,62,96,252,55,9,170,175,36,14,11,];
    let x0_c0_b2_a1_bytes = [189,156,0,235,163,90,36,226,124,135,231,181,119,172,9,171,212,53,232,31,193,188,40,186,228,71,128,43,21,97,254,245,137,234,155,125,218,241,206,42,136,184,220,122,164,26,18,23,];
    let x0_c1_b0_a0_bytes = [200,146,209,175,82,195,145,241,54,31,18,193,200,8,41,161,43,94,59,219,81,128,85,13,162,9,141,39,157,70,246,131,164,104,76,227,219,42,112,136,166,45,200,246,225,51,28,16,];
    let x0_c1_b0_a1_bytes = [54,115,148,26,219,101,46,245,26,216,90,142,45,183,28,250,222,213,38,96,62,92,225,241,52,207,25,59,75,34,131,253,200,155,159,146,254,106,174,192,21,208,115,104,89,82,201,12,];
    let x0_c1_b1_a0_bytes = [46,14,236,125,150,59,135,79,129,202,43,29,226,36,157,208,201,235,145,77,132,64,130,98,74,100,107,125,50,147,171,37,61,119,183,122,28,64,223,191,159,52,64,220,183,77,68,24,];
    let x0_c1_b1_a1_bytes = [120,70,77,94,71,235,65,233,161,74,206,155,203,39,168,202,136,61,64,186,114,75,137,76,47,131,84,47,137,223,249,64,195,103,21,145,78,20,37,241,150,118,48,64,106,50,197,1,];
    let x0_c1_b2_a0_bytes = [17,70,175,245,238,38,4,224,115,31,107,233,28,224,149,204,77,150,169,55,196,94,107,75,35,11,131,95,212,212,103,64,210,147,241,48,58,129,205,213,250,8,69,13,93,27,215,13,];
    let x0_c1_b2_a1_bytes = [42,34,192,185,113,199,199,165,168,0,80,76,229,232,229,191,97,111,8,96,226,177,83,192,195,209,33,216,64,40,10,244,85,12,215,16,249,93,55,53,217,94,24,147,149,76,113,6,];
    let x1_c0_b0_a0_bytes = [60,92,218,84,110,123,199,41,87,94,192,231,66,152,5,186,92,211,103,33,232,228,151,5,206,231,89,46,57,39,158,50,208,83,252,217,228,52,254,107,229,46,105,152,31,93,35,17,];
    let x1_c0_b0_a1_bytes = [106,251,2,54,89,25,70,97,241,184,44,143,138,187,197,209,110,166,22,156,71,37,31,87,29,181,17,61,83,135,73,230,255,106,77,58,230,157,180,41,5,26,227,40,196,78,186,17,];
    let x1_c0_b1_a0_bytes = [92,84,110,29,202,71,43,200,70,116,31,50,19,195,144,50,12,139,209,28,36,225,89,241,99,233,171,30,24,3,155,50,66,251,10,200,186,86,96,105,213,248,85,248,110,35,26,15,];
    let x1_c0_b1_a1_bytes = [173,116,187,196,213,153,240,42,151,106,69,11,251,231,152,77,136,117,57,154,178,108,49,165,171,24,80,207,93,16,90,195,135,66,214,92,73,4,104,238,29,167,252,105,52,81,23,22,];
    let x1_c0_b2_a0_bytes = [253,140,214,65,230,229,249,148,5,249,97,222,240,204,100,136,64,100,75,68,242,70,163,21,135,141,119,166,131,42,135,3,194,210,22,59,225,133,172,6,16,40,181,52,69,227,26,21,];
    let x1_c0_b2_a1_bytes = [137,181,69,64,102,26,114,215,0,254,8,156,53,38,158,33,146,155,37,52,246,157,120,135,96,158,208,90,4,175,163,68,23,3,241,72,20,104,92,28,13,67,243,77,23,215,179,19,];
    let x1_c1_b0_a0_bytes = [191,220,69,111,219,69,192,59,150,42,118,235,174,95,241,145,147,190,224,65,24,164,80,235,5,139,74,198,133,37,191,215,254,131,233,11,159,122,64,226,236,56,135,186,246,167,252,21,];
    let x1_c1_b0_a1_bytes = [108,243,84,77,223,98,25,156,113,210,47,53,192,254,227,74,12,183,85,153,146,247,161,172,86,65,68,123,204,144,221,107,98,46,176,204,146,72,63,145,71,177,139,186,180,139,12,6,];
    let x1_c1_b1_a0_bytes = [95,108,116,45,180,244,62,115,53,224,132,50,185,217,204,60,186,144,222,208,83,181,49,156,28,44,121,85,31,90,218,15,179,99,131,15,76,228,231,151,54,50,127,19,13,29,231,21,];
    let x1_c1_b1_a1_bytes = [208,84,155,33,71,227,55,60,166,69,70,175,217,19,65,151,96,229,196,237,185,71,127,24,116,26,180,160,101,9,181,128,127,140,20,237,51,116,229,87,4,70,219,177,136,38,190,10,];
    let x1_c1_b2_a0_bytes = [110,182,233,157,108,35,70,151,135,60,100,224,22,31,244,228,93,8,123,41,197,189,48,115,15,13,226,43,179,173,65,228,169,140,61,83,207,232,250,179,24,134,51,212,101,172,196,0,];
    let x1_c1_b2_a1_bytes = [23,226,188,161,124,0,174,246,12,60,212,16,30,23,148,45,120,66,11,61,225,76,178,199,73,143,156,121,137,33,85,79,171,168,197,87,245,121,93,254,29,223,214,163,159,182,77,25,];
    let x2_c0_b0_a0_bytes = [193,85,60,41,60,152,106,114,148,237,154,211,214,196,213,101,115,247,217,223,117,55,13,175,77,123,244,52,227,28,169,27,217,47,69,149,188,93,70,195,43,183,207,133,86,80,194,10,];
    let x2_c0_b0_a1_bytes = [36,127,151,163,201,85,223,30,16,103,144,95,65,225,213,110,31,137,215,101,254,117,77,161,242,65,131,175,78,158,70,195,181,212,1,41,189,131,187,191,33,51,232,34,165,99,97,4,];
    let x2_c0_b1_a0_bytes = [44,106,74,150,120,208,238,66,3,250,179,67,229,57,59,90,42,240,255,7,57,35,228,233,92,6,27,158,84,101,228,120,131,163,134,252,160,195,147,169,94,217,133,110,3,36,169,14,];
    let x2_c0_b1_a1_bytes = [207,75,223,255,56,145,37,87,131,151,214,99,155,236,192,39,57,184,80,4,204,139,105,209,89,221,48,231,216,143,50,106,51,240,179,216,42,92,12,208,162,59,252,106,187,52,78,14,];
    let x2_c0_b2_a0_bytes = [44,163,90,136,20,187,82,175,60,123,68,24,184,102,100,24,63,8,135,105,0,199,31,20,76,35,214,148,84,105,12,191,159,196,105,93,143,74,141,66,144,145,35,193,91,237,131,17,];
    let x2_c0_b2_a1_bytes = [0,57,213,117,115,227,33,33,242,96,162,92,199,126,170,210,90,42,239,201,182,137,254,147,209,115,88,138,184,7,209,171,204,145,116,8,81,149,240,199,215,224,91,183,175,14,114,24,];
    let x2_c1_b0_a0_bytes = [107,154,39,211,222,105,63,163,49,5,83,98,183,5,225,130,171,221,182,166,175,207,123,42,34,243,78,52,125,132,149,71,217,140,159,127,245,185,119,173,169,45,59,3,168,213,214,3,];
    let x2_c1_b0_a1_bytes = [41,123,78,190,56,110,2,65,52,247,49,179,167,29,231,228,230,200,225,201,125,207,251,92,191,56,173,61,137,11,175,65,228,18,121,196,134,228,2,210,12,3,33,212,17,25,4,20,];
    let x2_c1_b1_a0_bytes = [73,240,43,201,245,221,180,227,71,110,86,238,235,55,11,107,92,120,130,19,228,202,128,10,18,152,0,147,39,137,150,101,173,186,0,4,168,152,25,126,111,212,205,16,197,159,87,8,];
    let x2_c1_b1_a1_bytes = [241,9,83,199,86,120,96,84,72,214,186,152,30,128,230,207,67,248,15,247,245,117,250,32,214,193,219,69,24,112,89,102,226,19,43,231,198,14,141,1,110,7,177,148,133,72,114,13,];
    let x2_c1_b2_a0_bytes = [141,154,251,95,73,155,76,96,218,10,96,92,236,217,69,22,189,223,80,166,99,163,248,207,18,31,22,51,34,37,225,6,148,150,160,141,243,6,220,106,158,239,73,179,78,81,96,9,];
    let x2_c1_b2_a1_bytes = [251,124,170,135,94,22,235,110,117,182,48,254,114,133,34,113,83,69,102,241,200,233,124,188,239,165,178,171,57,37,214,60,30,131,116,44,118,206,190,85,20,118,212,69,194,20,81,16,];
    
    for i in 0..48 {
        assignment.a[0][0][0][i] = M31::from(x0_c0_b0_a0_bytes[i]);
        assignment.a[0][0][1][i] = M31::from(x0_c0_b0_a1_bytes[i]);
        assignment.a[0][1][0][i] = M31::from(x0_c0_b1_a0_bytes[i]);
        assignment.a[0][1][1][i] = M31::from(x0_c0_b1_a1_bytes[i]);
        assignment.a[0][2][0][i] = M31::from(x0_c0_b2_a0_bytes[i]);
        assignment.a[0][2][1][i] = M31::from(x0_c0_b2_a1_bytes[i]);
        assignment.a[1][0][0][i] = M31::from(x0_c1_b0_a0_bytes[i]);
        assignment.a[1][0][1][i] = M31::from(x0_c1_b0_a1_bytes[i]);
        assignment.a[1][1][0][i] = M31::from(x0_c1_b1_a0_bytes[i]);
        assignment.a[1][1][1][i] = M31::from(x0_c1_b1_a1_bytes[i]);
        assignment.a[1][2][0][i] = M31::from(x0_c1_b2_a0_bytes[i]);
        assignment.a[1][2][1][i] = M31::from(x0_c1_b2_a1_bytes[i]);
        assignment.b[0][0][0][i] = M31::from(x1_c0_b0_a0_bytes[i]);
        assignment.b[0][0][1][i] = M31::from(x1_c0_b0_a1_bytes[i]);
        assignment.b[0][1][0][i] = M31::from(x1_c0_b1_a0_bytes[i]);
        assignment.b[0][1][1][i] = M31::from(x1_c0_b1_a1_bytes[i]);
        assignment.b[0][2][0][i] = M31::from(x1_c0_b2_a0_bytes[i]);
        assignment.b[0][2][1][i] = M31::from(x1_c0_b2_a1_bytes[i]);
        assignment.b[1][0][0][i] = M31::from(x1_c1_b0_a0_bytes[i]);
        assignment.b[1][0][1][i] = M31::from(x1_c1_b0_a1_bytes[i]);
        assignment.b[1][1][0][i] = M31::from(x1_c1_b1_a0_bytes[i]);
        assignment.b[1][1][1][i] = M31::from(x1_c1_b1_a1_bytes[i]);
        assignment.b[1][2][0][i] = M31::from(x1_c1_b2_a0_bytes[i]);
        assignment.b[1][2][1][i] = M31::from(x1_c1_b2_a1_bytes[i]);
        assignment.c[0][0][0][i] = M31::from(x2_c0_b0_a0_bytes[i]);
        assignment.c[0][0][1][i] = M31::from(x2_c0_b0_a1_bytes[i]);
        assignment.c[0][1][0][i] = M31::from(x2_c0_b1_a0_bytes[i]);
        assignment.c[0][1][1][i] = M31::from(x2_c0_b1_a1_bytes[i]);
        assignment.c[0][2][0][i] = M31::from(x2_c0_b2_a0_bytes[i]);
        assignment.c[0][2][1][i] = M31::from(x2_c0_b2_a1_bytes[i]);
        assignment.c[1][0][0][i] = M31::from(x2_c1_b0_a0_bytes[i]);
        assignment.c[1][0][1][i] = M31::from(x2_c1_b0_a1_bytes[i]);
        assignment.c[1][1][0][i] = M31::from(x2_c1_b1_a0_bytes[i]);
        assignment.c[1][1][1][i] = M31::from(x2_c1_b1_a1_bytes[i]);
        assignment.c[1][2][0][i] = M31::from(x2_c1_b2_a0_bytes[i]);
        assignment.c[1][2][1][i] = M31::from(x2_c1_b2_a1_bytes[i]);
    }
    debug_eval(&E12DivCircuit::default(), &assignment, hint_registry);
}

declare_circuit!(E12SquareCircuit {
    a: [[[[Variable; 48]; 2]; 3]; 2],
    c: [[[[Variable; 48]; 2]; 3]; 2],
});

impl GenericDefine<M31Config> for E12SquareCircuit<Variable> {
    fn define<Builder: RootAPI<M31Config>>(&self, builder: &mut Builder) {
        let mut ext12 = Ext12::new(builder);

        let a_e12 = GE12 {
            c0: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.a[0][0][0].to_vec(), 0),
                    a1: new_internal_element(self.a[0][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.a[0][1][0].to_vec(), 0),
                    a1: new_internal_element(self.a[0][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.a[0][2][0].to_vec(), 0),
                    a1: new_internal_element(self.a[0][2][1].to_vec(), 0),
                },
            },
            c1: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.a[1][0][0].to_vec(), 0),
                    a1: new_internal_element(self.a[1][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.a[1][1][0].to_vec(), 0),
                    a1: new_internal_element(self.a[1][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.a[1][2][0].to_vec(), 0),
                    a1: new_internal_element(self.a[1][2][1].to_vec(), 0),
                },
            },
        };

        let c_e12 = GE12 {
            c0: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.c[0][0][0].to_vec(), 0),
                    a1: new_internal_element(self.c[0][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.c[0][1][0].to_vec(), 0),
                    a1: new_internal_element(self.c[0][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.c[0][2][0].to_vec(), 0),
                    a1: new_internal_element(self.c[0][2][1].to_vec(), 0),
                },
            },
            c1: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.c[1][0][0].to_vec(), 0),
                    a1: new_internal_element(self.c[1][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.c[1][1][0].to_vec(), 0),
                    a1: new_internal_element(self.c[1][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.c[1][2][0].to_vec(), 0),
                    a1: new_internal_element(self.c[1][2][1].to_vec(), 0),
                },
            },
        };

        let z = ext12.square(builder, &a_e12);
        ext12.assert_isequal(builder, &z, &c_e12);
        ext12.ext6.ext2.fp.check_mul(builder);
        ext12.ext6.ext2.fp.table.final_check(builder);
    }
}

#[test]
fn test_e12_square() {
    let compile_result =
        compile_generic(&E12SquareCircuit::default(), CompileOptions::default()).unwrap();
    let mut hint_registry = HintRegistry::<M31>::new();
    register_hint(&mut hint_registry);

    let mut assignment = E12SquareCircuit::<M31> {
        a: [[[[M31::from(0); 48]; 2]; 3]; 2],
        c: [[[[M31::from(0); 48]; 2]; 3]; 2],
    };
    let x0_c0_b0_a0_bytes = [88,133,252,130,248,35,113,86,1,233,243,26,171,123,147,247,95,0,7,89,214,56,125,216,216,127,82,24,54,235,55,222,80,208,90,30,69,10,30,120,48,239,117,55,217,64,92,3,];
    let x0_c0_b0_a1_bytes = [47,64,88,248,212,179,29,77,32,27,51,247,199,202,142,158,234,53,177,201,181,197,9,1,31,109,21,63,26,22,191,120,78,20,57,233,71,10,97,44,87,107,192,4,172,27,240,7,];
    let x0_c0_b1_a0_bytes = [111,64,203,144,84,246,36,84,242,40,158,185,116,81,136,56,251,133,233,214,83,122,228,55,216,140,109,26,132,43,108,73,117,38,229,19,179,243,194,140,171,145,49,72,198,113,51,3,];
    let x0_c0_b1_a1_bytes = [2,221,248,230,28,200,185,145,172,223,125,173,202,235,152,115,44,129,108,105,30,91,192,218,226,80,249,76,17,193,35,250,4,9,113,22,3,93,184,59,69,215,238,187,14,11,126,12,];
    let x0_c0_b2_a0_bytes = [27,66,201,99,213,78,185,239,188,95,52,87,91,2,47,201,133,144,37,59,95,204,68,241,81,241,17,237,119,31,105,139,9,146,5,39,56,173,211,225,43,100,93,64,31,193,100,10,];
    let x0_c0_b2_a1_bytes = [228,177,70,5,221,20,28,35,107,127,168,19,216,192,192,181,75,230,226,61,207,8,216,81,59,93,251,237,217,32,38,31,95,239,31,7,145,48,34,226,221,44,148,141,166,180,57,7,];
    let x0_c1_b0_a0_bytes = [33,25,52,14,225,200,176,33,108,144,161,200,90,168,64,62,88,113,62,78,211,132,185,129,131,61,99,106,157,96,28,164,122,234,91,235,157,10,45,85,72,219,225,17,132,159,195,5,];
    let x0_c1_b0_a1_bytes = [223,155,91,253,92,116,16,228,169,220,252,34,61,87,155,157,60,96,94,132,199,11,87,64,80,75,251,183,190,249,50,35,104,10,82,173,246,8,80,230,221,119,131,247,72,216,153,18,];
    let x0_c1_b1_a0_bytes = [250,77,130,197,255,70,2,248,42,12,139,237,212,143,76,125,58,221,126,44,217,108,8,44,150,215,153,92,49,204,179,33,8,83,253,253,229,92,72,29,153,131,175,39,242,89,235,12,];
    let x0_c1_b1_a1_bytes = [96,18,99,160,37,232,100,97,94,236,38,1,124,12,127,200,142,187,92,198,147,114,204,177,246,34,120,66,174,224,9,250,150,182,72,229,183,57,65,247,239,206,37,238,217,89,113,25,];
    let x0_c1_b2_a0_bytes = [86,113,59,186,59,194,185,19,155,48,222,99,52,213,161,32,61,208,232,126,193,112,193,226,67,195,78,127,121,178,125,13,230,244,75,177,128,121,245,106,83,157,242,30,200,116,51,10,];
    let x0_c1_b2_a1_bytes = [205,30,202,83,93,70,131,165,76,200,101,80,49,88,147,27,104,214,227,187,205,246,9,210,191,12,61,187,179,172,253,254,225,192,102,190,69,17,48,139,88,29,190,237,160,59,213,14,];
    let x2_c0_b0_a0_bytes = [71,158,226,94,15,60,102,52,213,157,153,47,92,130,187,97,53,22,93,208,27,134,165,158,166,222,70,179,83,210,55,113,161,158,96,191,132,115,16,164,235,215,203,8,202,111,164,3,];
    let x2_c0_b0_a1_bytes = [179,17,26,7,85,29,212,237,20,225,222,113,225,254,24,89,220,91,66,47,152,193,2,54,108,109,51,87,211,82,62,172,127,106,122,174,245,147,92,70,38,144,48,137,23,23,117,22,];
    let x2_c0_b1_a0_bytes = [149,111,12,131,79,201,24,186,92,70,254,36,2,125,222,214,235,139,219,116,105,235,108,63,81,142,61,218,32,17,138,25,183,233,98,216,36,229,68,9,135,245,251,153,91,52,129,20,];
    let x2_c0_b1_a1_bytes = [51,116,227,199,197,224,41,11,194,139,151,58,114,28,52,215,47,181,200,32,127,140,72,184,187,135,229,18,183,11,182,22,17,9,249,145,114,57,88,239,131,231,65,6,155,194,254,4,];
    let x2_c0_b2_a0_bytes = [83,243,249,17,182,3,187,178,50,163,228,7,41,42,112,214,49,230,209,51,47,231,202,159,207,53,206,156,185,78,41,218,53,51,150,34,225,3,70,109,175,0,196,203,223,250,72,23,];
    let x2_c0_b2_a1_bytes = [199,85,149,220,117,49,210,187,65,211,178,200,40,185,196,145,71,82,217,89,71,169,165,111,197,116,69,251,23,153,16,20,132,175,11,145,80,126,91,134,75,241,10,98,180,25,75,8,];
    let x2_c1_b0_a0_bytes = [141,236,203,10,202,77,75,56,220,209,236,228,179,193,0,11,150,176,93,11,160,247,196,42,124,7,17,177,63,114,152,248,70,54,208,219,105,251,220,155,234,26,196,108,114,133,30,15,];
    let x2_c1_b0_a1_bytes = [11,162,153,121,1,98,69,183,236,40,118,117,84,196,122,53,214,13,246,56,145,63,41,189,87,227,228,123,101,181,65,245,22,17,225,34,231,239,23,138,67,198,49,45,16,0,34,23,];
    let x2_c1_b1_a0_bytes = [121,71,222,182,82,106,82,68,121,64,189,104,112,119,219,131,92,81,73,12,67,128,130,243,98,74,171,126,252,134,58,25,252,128,244,180,125,86,217,76,33,252,223,237,162,185,29,10,];
    let x2_c1_b1_a1_bytes = [21,78,120,102,240,68,106,103,189,140,232,139,109,41,214,59,7,121,26,66,90,102,211,18,8,42,206,212,111,72,40,112,249,144,164,128,3,165,48,132,127,2,45,247,63,106,89,23,];
    let x2_c1_b2_a0_bytes = [139,58,122,68,234,250,127,30,253,71,195,108,110,86,70,100,190,112,72,165,128,16,212,8,59,173,66,56,168,153,20,11,212,98,254,27,216,204,202,169,121,168,120,226,241,209,132,0,];
    let x2_c1_b2_a1_bytes = [92,47,142,103,182,205,41,171,63,77,46,155,28,56,96,68,63,159,183,28,81,184,252,185,76,140,102,186,64,129,216,87,92,34,160,50,82,54,246,65,232,141,147,83,83,221,127,8,];

    for i in 0..48 {
        assignment.a[0][0][0][i] = M31::from(x0_c0_b0_a0_bytes[i]);
        assignment.a[0][0][1][i] = M31::from(x0_c0_b0_a1_bytes[i]);
        assignment.a[0][1][0][i] = M31::from(x0_c0_b1_a0_bytes[i]);
        assignment.a[0][1][1][i] = M31::from(x0_c0_b1_a1_bytes[i]);
        assignment.a[0][2][0][i] = M31::from(x0_c0_b2_a0_bytes[i]);
        assignment.a[0][2][1][i] = M31::from(x0_c0_b2_a1_bytes[i]);
        assignment.a[1][0][0][i] = M31::from(x0_c1_b0_a0_bytes[i]);
        assignment.a[1][0][1][i] = M31::from(x0_c1_b0_a1_bytes[i]);
        assignment.a[1][1][0][i] = M31::from(x0_c1_b1_a0_bytes[i]);
        assignment.a[1][1][1][i] = M31::from(x0_c1_b1_a1_bytes[i]);
        assignment.a[1][2][0][i] = M31::from(x0_c1_b2_a0_bytes[i]);
        assignment.a[1][2][1][i] = M31::from(x0_c1_b2_a1_bytes[i]);
        assignment.c[0][0][0][i] = M31::from(x2_c0_b0_a0_bytes[i]);
        assignment.c[0][0][1][i] = M31::from(x2_c0_b0_a1_bytes[i]);
        assignment.c[0][1][0][i] = M31::from(x2_c0_b1_a0_bytes[i]);
        assignment.c[0][1][1][i] = M31::from(x2_c0_b1_a1_bytes[i]);
        assignment.c[0][2][0][i] = M31::from(x2_c0_b2_a0_bytes[i]);
        assignment.c[0][2][1][i] = M31::from(x2_c0_b2_a1_bytes[i]);
        assignment.c[1][0][0][i] = M31::from(x2_c1_b0_a0_bytes[i]);
        assignment.c[1][0][1][i] = M31::from(x2_c1_b0_a1_bytes[i]);
        assignment.c[1][1][0][i] = M31::from(x2_c1_b1_a0_bytes[i]);
        assignment.c[1][1][1][i] = M31::from(x2_c1_b1_a1_bytes[i]);
        assignment.c[1][2][0][i] = M31::from(x2_c1_b2_a0_bytes[i]);
        assignment.c[1][2][1][i] = M31::from(x2_c1_b2_a1_bytes[i]);
    }

    debug_eval(&E12SquareCircuit::default(), &assignment, hint_registry);
}

declare_circuit!(E12ConjugateCircuit {
    a: [[[[Variable; 48]; 2]; 3]; 2],
    c: [[[[Variable; 48]; 2]; 3]; 2],
});

impl GenericDefine<M31Config> for E12ConjugateCircuit<Variable> {
    fn define<Builder: RootAPI<M31Config>>(&self, builder: &mut Builder) {
        let mut ext12 = Ext12::new(builder);

        let a_e12 = GE12 {
            c0: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.a[0][0][0].to_vec(), 0),
                    a1: new_internal_element(self.a[0][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.a[0][1][0].to_vec(), 0),
                    a1: new_internal_element(self.a[0][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.a[0][2][0].to_vec(), 0),
                    a1: new_internal_element(self.a[0][2][1].to_vec(), 0),
                },
            },
            c1: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.a[1][0][0].to_vec(), 0),
                    a1: new_internal_element(self.a[1][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.a[1][1][0].to_vec(), 0),
                    a1: new_internal_element(self.a[1][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.a[1][2][0].to_vec(), 0),
                    a1: new_internal_element(self.a[1][2][1].to_vec(), 0),
                },
            },
        };

        let c_e12 = GE12 {
            c0: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.c[0][0][0].to_vec(), 0),
                    a1: new_internal_element(self.c[0][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.c[0][1][0].to_vec(), 0),
                    a1: new_internal_element(self.c[0][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.c[0][2][0].to_vec(), 0),
                    a1: new_internal_element(self.c[0][2][1].to_vec(), 0),
                },
            },
            c1: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.c[1][0][0].to_vec(), 0),
                    a1: new_internal_element(self.c[1][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.c[1][1][0].to_vec(), 0),
                    a1: new_internal_element(self.c[1][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.c[1][2][0].to_vec(), 0),
                    a1: new_internal_element(self.c[1][2][1].to_vec(), 0),
                },
            },
        };

        let z = ext12.conjugate(builder, &a_e12);
        ext12.assert_isequal(builder, &z, &c_e12);
        ext12.ext6.ext2.fp.check_mul(builder);
        ext12.ext6.ext2.fp.table.final_check(builder);
    }
}

#[test]
fn test_e12_conjugate() {
    let compile_result =
        compile_generic(&E12ConjugateCircuit::default(), CompileOptions::default()).unwrap();
    let mut hint_registry = HintRegistry::<M31>::new();
    register_hint(&mut hint_registry);

    let mut assignment = E12ConjugateCircuit::<M31> {
        a: [[[[M31::from(0); 48]; 2]; 3]; 2],
        c: [[[[M31::from(0); 48]; 2]; 3]; 2],
    };
    let x0_c0_b0_a0_bytes = [71,177,236,83,1,17,168,246,122,100,204,112,142,217,145,27,117,59,181,4,229,102,112,231,144,76,212,114,160,6,240,191,127,58,84,179,120,206,111,94,23,146,65,115,219,104,57,7,];
    let x0_c0_b0_a1_bytes = [98,70,164,16,248,85,63,169,213,122,167,96,191,181,158,165,5,21,59,136,220,102,102,91,95,82,173,119,180,92,56,130,87,92,12,105,103,69,103,145,223,44,36,110,162,13,254,20,];
    let x0_c0_b1_a0_bytes = [55,212,190,91,232,203,217,72,223,44,237,68,48,180,74,228,203,178,114,41,178,72,186,81,112,129,254,48,20,251,238,215,62,167,155,163,75,120,212,115,165,23,78,10,208,29,139,18,];
    let x0_c0_b1_a1_bytes = [55,125,236,216,16,213,81,181,69,164,134,74,142,76,172,244,196,237,117,33,136,47,144,228,78,210,94,247,212,110,220,35,28,248,106,140,240,37,195,76,191,46,212,227,44,75,38,5,];
    let x0_c0_b2_a0_bytes = [108,135,79,73,222,246,223,3,196,88,96,97,246,150,37,39,189,31,83,226,241,117,168,182,37,40,84,61,167,84,169,98,124,99,203,2,251,90,140,51,191,75,138,35,75,61,10,14,];
    let x0_c0_b2_a1_bytes = [115,78,45,63,204,181,103,170,128,112,113,13,17,129,119,33,165,247,110,180,201,227,216,210,130,153,40,247,200,149,181,183,5,175,222,84,66,50,224,230,163,8,219,29,88,60,117,0,];
    let x0_c1_b0_a0_bytes = [135,152,139,17,161,56,2,200,103,224,8,28,89,75,246,96,113,142,12,114,129,93,114,50,98,235,194,5,255,19,176,190,238,241,217,155,94,110,35,223,208,121,202,45,36,228,191,1,];
    let x0_c1_b0_a1_bytes = [8,121,249,42,246,219,209,219,213,193,113,42,45,186,174,204,186,34,69,23,107,222,217,183,104,71,116,4,83,36,127,115,127,155,99,79,112,138,154,70,182,27,104,18,58,153,133,25,];
    let x0_c1_b1_a0_bytes = [243,206,55,0,101,194,150,200,220,120,221,22,96,108,9,91,132,137,197,247,86,186,43,155,181,94,160,171,96,172,158,111,54,155,88,2,238,135,35,144,225,43,226,46,73,116,171,11,];
    let x0_c1_b1_a1_bytes = [75,168,150,127,101,168,30,3,55,176,63,180,55,209,78,27,13,168,137,105,232,78,11,32,12,151,79,87,139,175,210,4,145,22,56,237,46,14,117,113,229,26,58,118,133,43,13,13,];
    let x0_c1_b2_a0_bytes = [156,21,251,228,85,140,169,144,214,200,194,238,194,169,249,223,17,86,36,172,183,194,241,22,28,130,174,104,241,241,85,132,33,109,84,66,149,250,181,179,232,160,93,201,167,65,56,4,];
    let x0_c1_b2_a1_bytes = [45,60,150,78,181,165,56,10,10,5,96,212,194,255,149,172,157,182,107,249,69,53,116,209,34,203,97,54,255,246,100,104,52,72,19,171,150,61,243,104,213,203,37,137,119,252,231,12,];
    let x2_c0_b0_a0_bytes = [71,177,236,83,1,17,168,246,122,100,204,112,142,217,145,27,117,59,181,4,229,102,112,231,144,76,212,114,160,6,240,191,127,58,84,179,120,206,111,94,23,146,65,115,219,104,57,7,];
    let x2_c0_b0_a1_bytes = [98,70,164,16,248,85,63,169,213,122,167,96,191,181,158,165,5,21,59,136,220,102,102,91,95,82,173,119,180,92,56,130,87,92,12,105,103,69,103,145,223,44,36,110,162,13,254,20,];
    let x2_c0_b1_a0_bytes = [55,212,190,91,232,203,217,72,223,44,237,68,48,180,74,228,203,178,114,41,178,72,186,81,112,129,254,48,20,251,238,215,62,167,155,163,75,120,212,115,165,23,78,10,208,29,139,18,];
    let x2_c0_b1_a1_bytes = [55,125,236,216,16,213,81,181,69,164,134,74,142,76,172,244,196,237,117,33,136,47,144,228,78,210,94,247,212,110,220,35,28,248,106,140,240,37,195,76,191,46,212,227,44,75,38,5,];
    let x2_c0_b2_a0_bytes = [108,135,79,73,222,246,223,3,196,88,96,97,246,150,37,39,189,31,83,226,241,117,168,182,37,40,84,61,167,84,169,98,124,99,203,2,251,90,140,51,191,75,138,35,75,61,10,14,];
    let x2_c0_b2_a1_bytes = [115,78,45,63,204,181,103,170,128,112,113,13,17,129,119,33,165,247,110,180,201,227,216,210,130,153,40,247,200,149,181,183,5,175,222,84,66,50,224,230,163,8,219,29,88,60,117,0,];
    let x2_c1_b0_a0_bytes = [36,18,116,238,94,199,252,241,151,31,75,149,165,180,181,189,178,103,164,132,31,117,190,52,93,39,194,237,133,55,199,165,232,186,113,167,87,57,248,107,201,108,181,11,198,45,65,24,];
    let x2_c1_b0_a1_bytes = [163,49,6,213,9,36,45,222,41,62,226,134,209,69,253,81,105,211,107,223,53,244,86,175,86,203,16,239,49,39,248,240,87,17,232,243,69,29,129,4,228,202,23,39,176,120,123,0,];
    let x2_c1_b1_a0_bytes = [184,219,199,255,154,61,104,241,34,135,118,154,158,147,162,195,159,108,235,254,73,24,5,204,9,180,228,71,36,159,216,244,160,17,243,64,200,31,248,186,184,186,157,10,161,157,85,14,];
    let x2_c1_b1_a1_bytes = [96,2,105,128,154,87,224,182,200,79,20,253,198,46,93,3,23,78,39,141,184,131,37,71,179,123,53,156,249,155,164,95,70,150,19,86,135,153,166,217,180,203,69,195,100,230,243,12,];
    let x2_c1_b2_a0_bytes = [15,149,4,27,170,115,85,41,41,55,145,194,59,86,178,62,18,160,140,74,233,15,63,80,163,144,214,138,147,89,33,224,181,63,247,0,33,173,101,151,177,69,34,112,66,208,200,21,];
    let x2_c1_b2_a1_bytes = [126,110,105,177,74,90,198,175,245,250,243,220,59,0,22,114,134,63,69,253,90,157,188,149,156,71,35,189,133,84,18,252,162,100,56,152,31,106,40,226,196,26,90,176,114,21,25,13,];

    for i in 0..48 {
        assignment.a[0][0][0][i] = M31::from(x0_c0_b0_a0_bytes[i]);
        assignment.a[0][0][1][i] = M31::from(x0_c0_b0_a1_bytes[i]);
        assignment.a[0][1][0][i] = M31::from(x0_c0_b1_a0_bytes[i]);
        assignment.a[0][1][1][i] = M31::from(x0_c0_b1_a1_bytes[i]);
        assignment.a[0][2][0][i] = M31::from(x0_c0_b2_a0_bytes[i]);
        assignment.a[0][2][1][i] = M31::from(x0_c0_b2_a1_bytes[i]);
        assignment.a[1][0][0][i] = M31::from(x0_c1_b0_a0_bytes[i]);
        assignment.a[1][0][1][i] = M31::from(x0_c1_b0_a1_bytes[i]);
        assignment.a[1][1][0][i] = M31::from(x0_c1_b1_a0_bytes[i]);
        assignment.a[1][1][1][i] = M31::from(x0_c1_b1_a1_bytes[i]);
        assignment.a[1][2][0][i] = M31::from(x0_c1_b2_a0_bytes[i]);
        assignment.a[1][2][1][i] = M31::from(x0_c1_b2_a1_bytes[i]);
        assignment.c[0][0][0][i] = M31::from(x2_c0_b0_a0_bytes[i]);
        assignment.c[0][0][1][i] = M31::from(x2_c0_b0_a1_bytes[i]);
        assignment.c[0][1][0][i] = M31::from(x2_c0_b1_a0_bytes[i]);
        assignment.c[0][1][1][i] = M31::from(x2_c0_b1_a1_bytes[i]);
        assignment.c[0][2][0][i] = M31::from(x2_c0_b2_a0_bytes[i]);
        assignment.c[0][2][1][i] = M31::from(x2_c0_b2_a1_bytes[i]);
        assignment.c[1][0][0][i] = M31::from(x2_c1_b0_a0_bytes[i]);
        assignment.c[1][0][1][i] = M31::from(x2_c1_b0_a1_bytes[i]);
        assignment.c[1][1][0][i] = M31::from(x2_c1_b1_a0_bytes[i]);
        assignment.c[1][1][1][i] = M31::from(x2_c1_b1_a1_bytes[i]);
        assignment.c[1][2][0][i] = M31::from(x2_c1_b2_a0_bytes[i]);
        assignment.c[1][2][1][i] = M31::from(x2_c1_b2_a1_bytes[i]);
    }

    debug_eval(&E12ConjugateCircuit::default(), &assignment, hint_registry);
}

declare_circuit!(E12InverseCircuit {
    a: [[[[Variable; 48]; 2]; 3]; 2],
    c: [[[[Variable; 48]; 2]; 3]; 2],
});

impl GenericDefine<M31Config> for E12InverseCircuit<Variable> {
    fn define<Builder: RootAPI<M31Config>>(&self, builder: &mut Builder) {
        let mut ext12 = Ext12::new(builder);

        let a_e12 = GE12 {
            c0: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.a[0][0][0].to_vec(), 0),
                    a1: new_internal_element(self.a[0][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.a[0][1][0].to_vec(), 0),
                    a1: new_internal_element(self.a[0][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.a[0][2][0].to_vec(), 0),
                    a1: new_internal_element(self.a[0][2][1].to_vec(), 0),
                },
            },
            c1: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.a[1][0][0].to_vec(), 0),
                    a1: new_internal_element(self.a[1][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.a[1][1][0].to_vec(), 0),
                    a1: new_internal_element(self.a[1][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.a[1][2][0].to_vec(), 0),
                    a1: new_internal_element(self.a[1][2][1].to_vec(), 0),
                },
            },
        };

        let c_e12 = GE12 {
            c0: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.c[0][0][0].to_vec(), 0),
                    a1: new_internal_element(self.c[0][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.c[0][1][0].to_vec(), 0),
                    a1: new_internal_element(self.c[0][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.c[0][2][0].to_vec(), 0),
                    a1: new_internal_element(self.c[0][2][1].to_vec(), 0),
                },
            },
            c1: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.c[1][0][0].to_vec(), 0),
                    a1: new_internal_element(self.c[1][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.c[1][1][0].to_vec(), 0),
                    a1: new_internal_element(self.c[1][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.c[1][2][0].to_vec(), 0),
                    a1: new_internal_element(self.c[1][2][1].to_vec(), 0),
                },
            },
        };

        let z = ext12.inverse(builder, &a_e12);
        ext12.assert_isequal(builder, &z, &c_e12);
        ext12.ext6.ext2.fp.check_mul(builder);
        ext12.ext6.ext2.fp.table.final_check(builder);
    }
}

#[test]
fn test_e12_inverse() {
    let compile_result =
        compile_generic(&E12InverseCircuit::default(), CompileOptions::default()).unwrap();
    let mut hint_registry = HintRegistry::<M31>::new();
    register_hint(&mut hint_registry);

    let mut assignment = E12InverseCircuit::<M31> {
        a: [[[[M31::from(0); 48]; 2]; 3]; 2],
        c: [[[[M31::from(0); 48]; 2]; 3]; 2],
    };
    let x0_c0_b0_a0_bytes = [239,186,91,151,236,129,147,153,101,99,53,151,162,197,14,129,206,52,82,66,40,93,181,127,159,109,86,10,123,147,115,119,236,230,242,84,19,56,246,198,89,111,151,230,140,35,172,23,];
    let x0_c0_b0_a1_bytes = [129,197,170,169,211,1,138,50,251,182,222,65,29,85,241,112,203,123,83,142,78,54,101,246,241,13,107,73,73,27,215,229,113,211,109,83,250,71,151,173,78,35,205,118,255,190,133,2,];
    let x0_c0_b1_a0_bytes = [213,43,183,161,86,92,215,70,136,235,36,130,5,48,66,116,93,226,131,54,211,42,44,129,95,197,114,157,128,111,237,159,42,235,82,225,113,134,63,128,68,138,243,118,58,154,85,23,];
    let x0_c0_b1_a1_bytes = [114,226,223,71,191,8,71,98,212,201,134,67,17,67,112,72,13,33,13,224,6,172,231,177,160,227,217,230,147,22,70,71,125,239,212,160,161,245,34,195,37,117,140,115,217,166,1,12,];
    let x0_c0_b2_a0_bytes = [173,62,209,5,189,147,109,62,65,158,66,54,136,251,249,122,50,122,70,119,226,158,12,244,61,175,69,95,78,101,28,103,42,21,43,254,0,183,162,17,202,212,97,232,169,231,31,6,];
    let x0_c0_b2_a1_bytes = [102,4,179,120,17,221,42,212,239,7,7,31,186,185,3,44,237,22,250,85,111,94,226,138,111,134,175,237,55,208,37,210,231,8,254,247,196,61,138,81,208,158,27,122,37,166,58,14,];
    let x0_c1_b0_a0_bytes = [68,117,204,86,188,131,76,39,232,170,1,168,214,0,211,16,139,169,39,58,251,138,210,214,10,95,209,138,91,65,161,116,191,111,56,130,80,38,168,232,117,1,73,115,124,171,43,11,];
    let x0_c1_b0_a1_bytes = [7,122,155,89,246,186,116,55,46,146,121,114,185,240,212,116,96,14,145,133,36,128,156,208,153,122,95,170,97,83,156,180,196,193,166,73,128,146,146,20,250,6,91,179,83,233,79,17,];
    let x0_c1_b1_a0_bytes = [54,148,249,115,176,147,190,102,19,199,129,72,19,255,35,66,35,39,139,124,233,5,56,74,211,196,116,80,177,184,65,142,219,129,2,214,251,11,61,231,142,103,194,34,114,204,241,18,];
    let x0_c1_b1_a1_bytes = [149,115,220,144,24,182,223,191,4,238,199,71,115,98,97,148,102,62,143,18,71,27,64,213,180,149,53,153,46,192,74,169,109,199,19,27,247,92,194,209,115,88,36,43,23,235,99,3,];
    let x0_c1_b2_a0_bytes = [207,64,86,239,93,197,185,192,250,176,52,113,5,9,141,195,16,43,42,138,200,149,95,121,15,125,71,119,141,68,215,140,2,220,57,6,73,21,185,32,111,5,235,41,136,124,143,10,];
    let x0_c1_b2_a1_bytes = [163,180,236,225,210,55,0,151,126,111,86,98,207,29,45,229,123,119,174,140,120,117,78,237,155,193,218,54,191,241,33,5,145,169,207,165,84,25,99,106,93,124,150,93,43,46,25,2,];
    let x2_c0_b0_a0_bytes = [57,214,182,130,35,159,250,24,209,249,80,73,243,134,169,163,114,248,153,112,127,226,230,68,197,234,100,109,111,98,238,0,214,165,110,228,34,255,243,76,107,48,226,17,93,223,138,7,];
    let x2_c0_b0_a1_bytes = [161,146,144,233,77,212,55,2,104,132,98,221,178,21,102,5,108,47,242,77,97,196,63,16,232,62,255,69,229,213,80,32,191,163,15,40,94,56,112,207,110,239,148,161,222,178,210,24,];
    let x2_c0_b1_a0_bytes = [89,67,10,79,236,37,119,218,66,177,21,220,69,153,231,145,242,6,110,247,155,53,163,68,134,161,21,182,60,156,127,205,125,126,113,112,7,44,193,129,104,203,241,240,114,100,189,18,];
    let x2_c0_b1_a1_bytes = [86,135,71,239,167,1,39,92,175,78,24,72,242,186,239,252,243,182,155,181,254,11,202,187,134,137,139,112,249,252,164,178,32,149,88,48,171,167,198,56,242,47,161,83,184,99,20,13,];
    let x2_c0_b2_a0_bytes = [119,10,21,35,53,171,73,201,190,67,49,86,58,77,247,76,80,240,12,59,8,89,147,164,147,54,211,62,114,137,64,39,186,240,252,134,109,255,125,101,97,89,71,44,115,120,233,24,];
    let x2_c0_b2_a1_bytes = [182,232,20,90,71,192,139,141,111,157,143,24,204,150,173,203,139,134,130,160,171,135,20,204,236,150,25,223,43,37,145,212,102,207,204,32,78,142,23,44,79,8,42,199,176,105,208,8,];
    let x2_c1_b0_a0_bytes = [177,251,61,25,122,5,17,207,251,43,55,10,247,253,31,163,175,201,61,254,47,144,137,204,83,57,178,171,255,69,153,178,165,217,113,28,235,33,203,6,207,251,85,32,219,4,161,15,];
    let x2_c1_b0_a1_bytes = [224,185,252,67,17,11,212,145,15,21,53,184,30,147,28,140,61,193,213,87,132,221,11,125,69,105,73,204,152,156,134,106,210,73,189,209,109,164,161,232,241,171,183,123,243,240,69,17,];
    let x2_c1_b1_a0_bytes = [210,222,123,144,12,44,162,17,183,202,81,141,237,186,74,145,60,11,235,203,217,207,77,119,54,162,37,122,37,125,203,106,192,193,198,216,102,173,152,126,29,217,26,101,71,28,71,12,];
    let x2_c1_b1_a1_bytes = [127,8,161,3,209,235,42,144,140,233,109,196,17,15,62,139,56,181,19,120,176,247,44,34,155,222,189,228,93,70,24,167,83,250,171,150,195,194,212,136,247,103,205,104,87,227,41,10,];
    let x2_c1_b2_a0_bytes = [149,146,83,190,252,159,164,10,252,95,197,72,197,222,22,150,236,47,242,28,19,182,100,118,242,41,87,156,192,146,219,11,150,114,7,140,84,132,57,98,151,187,49,172,0,154,158,4,];
    let x2_c1_b2_a1_bytes = [89,197,129,4,34,216,120,179,250,172,172,26,57,188,253,93,68,213,85,156,232,216,158,222,243,177,243,162,177,230,118,217,138,9,135,45,160,84,233,110,67,47,104,250,232,222,121,0,];

    for i in 0..48 {
        assignment.a[0][0][0][i] = M31::from(x0_c0_b0_a0_bytes[i]);
        assignment.a[0][0][1][i] = M31::from(x0_c0_b0_a1_bytes[i]);
        assignment.a[0][1][0][i] = M31::from(x0_c0_b1_a0_bytes[i]);
        assignment.a[0][1][1][i] = M31::from(x0_c0_b1_a1_bytes[i]);
        assignment.a[0][2][0][i] = M31::from(x0_c0_b2_a0_bytes[i]);
        assignment.a[0][2][1][i] = M31::from(x0_c0_b2_a1_bytes[i]);
        assignment.a[1][0][0][i] = M31::from(x0_c1_b0_a0_bytes[i]);
        assignment.a[1][0][1][i] = M31::from(x0_c1_b0_a1_bytes[i]);
        assignment.a[1][1][0][i] = M31::from(x0_c1_b1_a0_bytes[i]);
        assignment.a[1][1][1][i] = M31::from(x0_c1_b1_a1_bytes[i]);
        assignment.a[1][2][0][i] = M31::from(x0_c1_b2_a0_bytes[i]);
        assignment.a[1][2][1][i] = M31::from(x0_c1_b2_a1_bytes[i]);
        assignment.c[0][0][0][i] = M31::from(x2_c0_b0_a0_bytes[i]);
        assignment.c[0][0][1][i] = M31::from(x2_c0_b0_a1_bytes[i]);
        assignment.c[0][1][0][i] = M31::from(x2_c0_b1_a0_bytes[i]);
        assignment.c[0][1][1][i] = M31::from(x2_c0_b1_a1_bytes[i]);
        assignment.c[0][2][0][i] = M31::from(x2_c0_b2_a0_bytes[i]);
        assignment.c[0][2][1][i] = M31::from(x2_c0_b2_a1_bytes[i]);
        assignment.c[1][0][0][i] = M31::from(x2_c1_b0_a0_bytes[i]);
        assignment.c[1][0][1][i] = M31::from(x2_c1_b0_a1_bytes[i]);
        assignment.c[1][1][0][i] = M31::from(x2_c1_b1_a0_bytes[i]);
        assignment.c[1][1][1][i] = M31::from(x2_c1_b1_a1_bytes[i]);
        assignment.c[1][2][0][i] = M31::from(x2_c1_b2_a0_bytes[i]);
        assignment.c[1][2][1][i] = M31::from(x2_c1_b2_a1_bytes[i]);
    }
    debug_eval(&E12InverseCircuit::default(), &assignment, hint_registry);
}

declare_circuit!(E12MulBy014Circuit {
    a: [[[[Variable; 48]; 2]; 3]; 2],
    w: [[[[Variable; 48]; 2]; 3]; 2],
    b: [[Variable; 48]; 2],
    c: [[Variable; 48]; 2],
});

impl GenericDefine<M31Config> for E12MulBy014Circuit<Variable> {
    fn define<Builder: RootAPI<M31Config>>(&self, builder: &mut Builder) {
        let mut ext12 = Ext12::new(builder);

        let a_e12 = GE12 {
            c0: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.a[0][0][0].to_vec(), 0),
                    a1: new_internal_element(self.a[0][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.a[0][1][0].to_vec(), 0),
                    a1: new_internal_element(self.a[0][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.a[0][2][0].to_vec(), 0),
                    a1: new_internal_element(self.a[0][2][1].to_vec(), 0),
                },
            },
            c1: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.a[1][0][0].to_vec(), 0),
                    a1: new_internal_element(self.a[1][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.a[1][1][0].to_vec(), 0),
                    a1: new_internal_element(self.a[1][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.a[1][2][0].to_vec(), 0),
                    a1: new_internal_element(self.a[1][2][1].to_vec(), 0),
                },
            },
        };

        let w_e12 = GE12 {
            c0: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.w[0][0][0].to_vec(), 0),
                    a1: new_internal_element(self.w[0][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.w[0][1][0].to_vec(), 0),
                    a1: new_internal_element(self.w[0][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.w[0][2][0].to_vec(), 0),
                    a1: new_internal_element(self.w[0][2][1].to_vec(), 0),
                },
            },
            c1: GE6 {
                b0: GE2 {
                    a0: new_internal_element(self.w[1][0][0].to_vec(), 0),
                    a1: new_internal_element(self.w[1][0][1].to_vec(), 0),
                },
                b1: GE2 {
                    a0: new_internal_element(self.w[1][1][0].to_vec(), 0),
                    a1: new_internal_element(self.w[1][1][1].to_vec(), 0),
                },
                b2: GE2 {
                    a0: new_internal_element(self.w[1][2][0].to_vec(), 0),
                    a1: new_internal_element(self.w[1][2][1].to_vec(), 0),
                },
            },
        };

        let b_e2 = GE2 {
            a0: new_internal_element(self.b[0].to_vec(), 0),
            a1: new_internal_element(self.b[1].to_vec(), 0),
        };

        let c_e2 = GE2 {
            a0: new_internal_element(self.c[0].to_vec(), 0),
            a1: new_internal_element(self.c[1].to_vec(), 0),
        };

        let z = ext12.mul_by_014(builder, &a_e12, &b_e2, &c_e2);
        ext12.assert_isequal(builder, &z, &w_e12);
        ext12.ext6.ext2.fp.check_mul(builder);
        ext12.ext6.ext2.fp.table.final_check(builder);
    }
}

#[test]
fn test_e12_mul_by_014() {
    // let compile_result =
        // compile_generic(&E12MulBy014Circuit::default(), CompileOptions::default()).unwrap();
    let mut hint_registry = HintRegistry::<M31>::new();
    register_hint(&mut hint_registry);

    let mut assignment = E12MulBy014Circuit::<M31> {
        a: [[[[M31::from(0); 48]; 2]; 3]; 2],
        w: [[[[M31::from(0); 48]; 2]; 3]; 2],
        b: [[M31::from(0); 48]; 2],
        c: [[M31::from(0); 48]; 2],
    };
    let x0_c0_b0_a0_bytes = [46,225,141,72,79,6,52,59,209,213,86,160,220,208,132,110,53,70,111,237,250,13,135,108,93,27,196,125,229,194,108,221,127,4,115,130,225,243,250,188,89,102,164,141,191,208,246,22,];
    let x0_c0_b0_a1_bytes = [31,107,172,201,84,5,66,186,151,71,249,145,228,59,45,212,200,223,1,16,229,57,250,233,212,35,187,34,118,226,250,125,125,173,6,187,2,234,253,112,193,250,181,214,49,29,150,22,];
    let x0_c0_b1_a0_bytes = [102,132,113,1,157,235,122,46,89,173,53,254,78,47,128,55,205,137,5,222,247,82,1,250,59,129,8,180,128,183,28,9,111,191,183,115,239,27,222,239,238,61,74,8,57,100,87,14,];
    let x0_c0_b1_a1_bytes = [211,198,117,79,222,237,57,94,161,82,233,228,137,153,45,193,238,255,73,106,208,95,16,191,145,216,253,216,63,176,145,77,179,252,234,60,4,184,71,22,19,70,176,90,243,27,190,13,];
    let x0_c0_b2_a0_bytes = [151,168,135,95,89,100,143,171,239,191,150,12,80,189,237,24,22,155,221,154,95,234,83,226,158,222,54,60,182,225,240,29,122,81,228,72,240,76,243,94,198,255,8,19,222,224,137,21,];
    let x0_c0_b2_a1_bytes = [28,112,79,97,105,30,99,190,237,253,96,11,23,52,152,45,155,53,10,47,6,39,119,166,156,107,163,207,226,140,64,65,96,200,95,201,13,55,127,136,55,9,123,33,67,0,158,21,];
    let x0_c1_b0_a0_bytes = [212,171,88,128,53,43,171,112,143,58,210,187,196,137,38,89,57,223,27,124,231,24,0,187,204,189,55,104,249,111,68,82,11,127,112,65,163,142,48,175,61,165,140,94,7,93,134,23,];
    let x0_c1_b0_a1_bytes = [69,70,146,4,112,110,61,229,87,7,88,244,130,214,149,194,13,228,203,135,25,62,35,215,158,227,144,239,67,100,10,250,22,57,183,186,56,197,235,11,44,103,198,44,169,66,41,6,];
    let x0_c1_b1_a0_bytes = [116,164,31,12,98,150,12,73,229,235,76,171,164,90,119,217,95,2,213,201,107,68,44,233,66,236,251,36,209,84,101,16,39,100,113,12,173,46,113,75,99,150,80,82,216,89,173,11,];
    let x0_c1_b1_a1_bytes = [212,231,52,254,7,77,81,168,142,65,198,223,119,200,170,39,62,180,161,52,229,96,188,148,59,205,34,160,235,54,180,242,166,165,80,213,187,178,112,41,236,98,135,190,50,87,148,17,];
    let x0_c1_b2_a0_bytes = [203,2,160,135,190,99,216,217,114,53,245,58,73,240,132,99,109,175,162,114,96,150,248,105,216,12,205,67,121,31,105,68,189,49,20,110,8,108,146,5,248,7,36,205,153,144,33,13,];
    let x0_c1_b2_a1_bytes = [203,136,84,84,75,168,160,42,254,245,246,224,74,54,92,224,184,237,123,60,155,213,237,99,78,84,82,187,38,238,213,213,150,148,186,89,137,174,204,235,236,253,12,2,84,47,121,10,];
    let x1_a0_bytes = [97,217,42,113,196,20,178,27,215,13,156,167,138,17,171,196,232,155,154,149,209,178,84,234,115,240,69,32,234,186,21,219,82,254,108,18,101,227,82,125,231,36,240,88,221,86,203,4,];
    let x1_a1_bytes = [181,119,85,130,130,97,98,37,183,64,108,80,157,44,213,158,31,115,18,140,43,129,7,96,201,228,58,17,72,80,38,60,222,6,243,230,151,157,15,199,64,204,251,199,87,114,30,14,];
    let x2_a0_bytes = [142,57,191,139,145,59,244,144,145,73,235,127,111,15,212,26,156,71,198,192,110,63,33,64,132,28,22,180,142,188,167,105,90,169,73,42,100,218,78,81,162,17,252,88,132,34,36,25,];
    let x2_a1_bytes = [141,172,175,31,128,169,179,227,202,136,6,176,193,155,72,63,72,69,49,75,204,13,77,41,90,208,48,109,251,81,88,232,104,211,141,6,146,48,156,255,102,143,17,169,187,25,164,24,];
    let x3_c0_b0_a0_bytes = [139,193,89,3,233,201,122,223,194,169,54,194,48,252,80,208,78,220,230,21,0,245,152,35,53,51,57,175,145,231,17,100,230,199,48,3,91,7,51,3,201,191,182,179,127,245,84,22,];
    let x3_c0_b0_a1_bytes = [143,137,64,149,139,89,220,39,12,127,45,136,61,41,159,67,114,127,252,46,20,121,136,49,88,130,161,80,103,23,73,179,59,221,18,162,143,167,85,43,54,92,223,169,48,23,33,13,];
    let x3_c0_b1_a0_bytes = [218,58,2,251,106,226,165,205,132,234,252,159,96,3,66,52,135,235,35,245,178,53,125,139,37,161,93,201,234,166,231,137,2,46,84,203,210,63,135,22,39,121,217,49,195,178,109,13,];
    let x3_c0_b1_a1_bytes = [69,81,11,211,140,63,176,144,200,183,213,228,47,4,188,80,145,7,70,41,127,13,90,22,44,221,197,66,237,119,132,158,164,38,247,160,217,173,103,2,227,124,246,225,247,237,70,8,];
    let x3_c0_b2_a0_bytes = [213,70,9,166,158,52,110,129,50,212,141,195,222,84,123,45,199,68,201,227,209,120,57,73,231,101,30,138,183,8,48,53,71,37,251,64,241,72,16,136,174,60,196,26,204,252,254,16,];
    let x3_c0_b2_a1_bytes = [92,75,160,53,232,125,245,45,81,16,110,36,179,125,207,188,190,45,100,167,24,74,103,225,158,87,184,194,198,69,15,77,142,228,157,196,111,103,84,244,167,53,118,185,177,119,212,23,];
    let x3_c1_b0_a0_bytes = [79,180,128,190,186,98,168,175,124,93,72,97,41,254,186,145,181,2,3,99,19,243,187,225,99,96,108,143,214,4,119,79,171,52,55,3,240,237,207,179,186,129,67,225,190,53,232,5,];
    let x3_c1_b0_a1_bytes = [101,50,45,138,153,115,140,5,53,2,165,107,108,181,19,195,66,84,132,120,144,67,247,39,47,0,32,226,132,40,109,58,69,196,160,249,51,240,102,156,13,85,69,252,91,12,10,0,];
    let x3_c1_b1_a0_bytes = [148,187,155,201,27,246,72,5,110,230,145,147,78,48,217,232,208,216,193,55,149,123,211,76,177,184,136,97,171,210,173,128,212,119,192,0,128,8,157,49,248,39,179,185,226,163,81,18,];
    let x3_c1_b1_a1_bytes = [1,157,251,4,189,95,113,234,155,50,0,251,38,171,221,139,75,188,130,49,177,148,232,100,251,64,90,167,177,187,140,234,43,133,148,174,104,4,12,65,237,37,45,125,68,64,239,6,];
    let x3_c1_b2_a0_bytes = [199,44,149,165,101,136,132,147,162,147,239,173,253,64,189,26,139,51,208,95,216,1,193,161,199,211,25,240,43,126,189,172,166,101,10,165,218,25,170,24,167,87,240,13,45,62,111,23,];
    let x3_c1_b2_a1_bytes = [205,79,236,205,166,11,179,69,160,45,40,178,191,234,149,228,61,98,86,83,162,219,49,32,134,142,185,213,255,225,114,198,88,86,22,229,93,24,197,179,155,224,134,14,203,213,114,8,];

    for i in 0..48 {
        assignment.a[0][0][0][i] = M31::from(x0_c0_b0_a0_bytes[i]);
        assignment.a[0][0][1][i] = M31::from(x0_c0_b0_a1_bytes[i]);
        assignment.a[0][1][0][i] = M31::from(x0_c0_b1_a0_bytes[i]);
        assignment.a[0][1][1][i] = M31::from(x0_c0_b1_a1_bytes[i]);
        assignment.a[0][2][0][i] = M31::from(x0_c0_b2_a0_bytes[i]);
        assignment.a[0][2][1][i] = M31::from(x0_c0_b2_a1_bytes[i]);
        assignment.a[1][0][0][i] = M31::from(x0_c1_b0_a0_bytes[i]);
        assignment.a[1][0][1][i] = M31::from(x0_c1_b0_a1_bytes[i]);
        assignment.a[1][1][0][i] = M31::from(x0_c1_b1_a0_bytes[i]);
        assignment.a[1][1][1][i] = M31::from(x0_c1_b1_a1_bytes[i]);
        assignment.a[1][2][0][i] = M31::from(x0_c1_b2_a0_bytes[i]);
        assignment.a[1][2][1][i] = M31::from(x0_c1_b2_a1_bytes[i]);
        assignment.b[0][i] = M31::from(x1_a0_bytes[i]);
        assignment.b[1][i] = M31::from(x1_a1_bytes[i]);
        assignment.c[0][i] = M31::from(x2_a0_bytes[i]);
        assignment.c[1][i] = M31::from(x2_a1_bytes[i]);
        assignment.w[0][0][0][i] = M31::from(x3_c0_b0_a0_bytes[i]);
        assignment.w[0][0][1][i] = M31::from(x3_c0_b0_a1_bytes[i]);
        assignment.w[0][1][0][i] = M31::from(x3_c0_b1_a0_bytes[i]);
        assignment.w[0][1][1][i] = M31::from(x3_c0_b1_a1_bytes[i]);
        assignment.w[0][2][0][i] = M31::from(x3_c0_b2_a0_bytes[i]);
        assignment.w[0][2][1][i] = M31::from(x3_c0_b2_a1_bytes[i]);
        assignment.w[1][0][0][i] = M31::from(x3_c1_b0_a0_bytes[i]);
        assignment.w[1][0][1][i] = M31::from(x3_c1_b0_a1_bytes[i]);
        assignment.w[1][1][0][i] = M31::from(x3_c1_b1_a0_bytes[i]);
        assignment.w[1][1][1][i] = M31::from(x3_c1_b1_a1_bytes[i]);
        assignment.w[1][2][0][i] = M31::from(x3_c1_b2_a0_bytes[i]);
        assignment.w[1][2][1][i] = M31::from(x3_c1_b2_a1_bytes[i]);
    }
    debug_eval(&E12MulBy014Circuit::default(), &assignment, hint_registry);
}


// pub fn print_e2<'a, C:Config, B:RootAPI<C>>(native: &'a mut B, v: &GE2)  {
//     print_element(native, &v.a0);
//     print_element(native, &v.a1);
// }
// pub fn print_element<'a, C:Config, B:RootAPI<C>, T: FieldParams>(native: &'a mut B, v: &Element<T>)  {
//     for i in 0..48 {
//         print!("{:?}", native.value_of(v.limbs[i]));
//     }
//     println!();
// }