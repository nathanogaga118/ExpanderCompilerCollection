use std::thread;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use ark_bls12_381::g2;
use circuit_std_rs::gnark::hints::register_hint;
use expander_compiler::frontend::*;
use expander_config::M31ExtConfigSha2;
use num_bigint::BigInt;
use sha2::{Digest, Sha256};
use circuit_std_rs::big_int::{to_binary_hint, big_array_add};
use circuit_std_rs::sha2_m31::check_sha256;
use circuit_std_rs::gnark::emulated::field_bls12381::*;
use circuit_std_rs::gnark::emulated::field_bls12381::e2::*;
use circuit_std_rs::gnark::emulated::sw_bls12381::pairing::*;
use circuit_std_rs::gnark::emulated::sw_bls12381::g1::*;
use circuit_std_rs::gnark::emulated::sw_bls12381::g2::*;
use circuit_std_rs::gnark::element::*;
use expander_compiler::frontend::extra::*;
use circuit_std_rs::big_int::*;
use expander_compiler::{circuit::layered::InputType, frontend::*};

use crate::utils::run_circuit;

declare_circuit!(PairingCircuit {
    pubkey: [[Variable;48];2],
    hm: [[[Variable;48];2];2],
    sig: [[[Variable;48];2];2],
});

impl GenericDefine<M31Config> for PairingCircuit<Variable> {
    fn define<Builder: RootAPI<M31Config>>(&self, builder: &mut Builder) {
        let mut pairing = Pairing::new(builder);
        let one_g1 = G1Affine::one(builder);
        let pubkey_g1 = G1Affine::from_vars(self.pubkey[0].to_vec(), self.pubkey[1].to_vec());
        let hm_g2 = G2AffP::from_vars(self.hm[0][0].to_vec(), self.hm[0][1].to_vec(), self.hm[1][0].to_vec(), self.hm[1][1].to_vec());
        let sig_g2 = G2AffP::from_vars(self.sig[0][0].to_vec(), self.sig[0][1].to_vec(), self.sig[1][0].to_vec(), self.sig[1][1].to_vec());

        let mut g2 = G2::new(builder);
        let neg_sig_g2 = g2.neg(builder, &sig_g2);

        // P := []*G1Affine{&one_g1, &pubkey_g1}
        // Q := []*G2Affine{neg_sig_g2, &hm_g2}
        // pairing.pairingcheck(P, Q)
        let p_array = vec![one_g1, pubkey_g1];
        let mut q_array = [G2Affine{p: neg_sig_g2, lines: LineEvaluations::default()}, G2Affine{p: hm_g2, lines: LineEvaluations::default()}];
        pairing.pairing_check(builder, &p_array, &mut q_array);
        pairing.ext12.ext6.ext2.fp.check_mul(builder);
        pairing.ext12.ext6.ext2.fp.table.final_check(builder);
    }
}



#[test]
fn test_pairing_check_gkr() {
    // let compile_result =
    // compile_generic(&PairingCircuit::default(), CompileOptions::default()).unwrap();
    let mut hint_registry = HintRegistry::<M31>::new();
    register_hint(&mut hint_registry);
    /*
    hm E([2128747184964102066453428909345807587167353354433686779055175069717994597853044053001604474195549116663962354781667+600928199043548865756890420428378235956589666349872943435617471245143322438124492345775032317976373712791854412075*u,2673014212711484998033216133821539885421138070306477264866327549730911573831074801525177859765712567167095903919303+843401639836709482028685764607129261791330643868212867532430090507242037514006427793603581220496836139166547085499*u])
    sig E([963823355633972122114533498175662916621992470505354782789337615847591161145194281419366975300935939968232579346290+596907481049847637954275493859228934805964488037826922094320375977359016208358247522168009186501678750789366694831*u,1503040898615551538476187079486863259539849948567091887110583169943865184109068018840042625482669131770515482621711+3444166137003222945962463909857562676481832034105318967013156342862358108020440293426901361538632823324929201906078*u])
    aggPubkey E([3103244252149090420124940058491173358275189586453938010595576928631997313493844448363005953641905183987079560513835,1296246409150097609953508557969533080097715407458068120115474713311006715865163545587973784795351244083056720382121])
     */
    let assignment = PairingCircuit::<M31> {
        pubkey: [string_to_m31_array("3103244252149090420124940058491173358275189586453938010595576928631997313493844448363005953641905183987079560513835", 8), 
                string_to_m31_array("1296246409150097609953508557969533080097715407458068120115474713311006715865163545587973784795351244083056720382121", 8)],
        hm: [
            [string_to_m31_array("2128747184964102066453428909345807587167353354433686779055175069717994597853044053001604474195549116663962354781667", 8), 
            string_to_m31_array("600928199043548865756890420428378235956589666349872943435617471245143322438124492345775032317976373712791854412075", 8)], 
            [string_to_m31_array("2673014212711484998033216133821539885421138070306477264866327549730911573831074801525177859765712567167095903919303", 8),
            string_to_m31_array("843401639836709482028685764607129261791330643868212867532430090507242037514006427793603581220496836139166547085499", 8)]
            ],
        sig: [
            [string_to_m31_array("963823355633972122114533498175662916621992470505354782789337615847591161145194281419366975300935939968232579346290", 8), 
            string_to_m31_array("596907481049847637954275493859228934805964488037826922094320375977359016208358247522168009186501678750789366694831", 8),],
            [string_to_m31_array("1503040898615551538476187079486863259539849948567091887110583169943865184109068018840042625482669131770515482621711", 8),
            string_to_m31_array("3444166137003222945962463909857562676481832034105318967013156342862358108020440293426901361538632823324929201906078", 8)]
        ]
    };
    println!("assignment.pubkey[0]: {:?}", assignment.pubkey[0]);
    println!("assignment.pubkey[1]: {:?}", assignment.pubkey[1]);
    println!("assignment.hm[0][0]: {:?}", assignment.hm[0][0]);
    println!("assignment.hm[0][1]: {:?}", assignment.hm[0][1]);
    println!("assignment.hm[1][0]: {:?}", assignment.hm[1][0]);
    println!("assignment.hm[1][1]: {:?}", assignment.hm[1][1]);
    println!("assignment.sig[0][0]: {:?}", assignment.sig[0][0]);
    println!("assignment.sig[0][1]: {:?}", assignment.sig[0][1]);
    println!("assignment.sig[1][0]: {:?}", assignment.sig[1][0]);
    println!("assignment.sig[1][1]: {:?}", assignment.sig[1][1]);
    debug_eval(&PairingCircuit::default(), &assignment, hint_registry);
}



#[test]
fn run_expander_pairing(){
	let mut hint_registry = HintRegistry::<M31>::new();
    register_hint(&mut hint_registry);
    /*
    hm E([2128747184964102066453428909345807587167353354433686779055175069717994597853044053001604474195549116663962354781667+600928199043548865756890420428378235956589666349872943435617471245143322438124492345775032317976373712791854412075*u,2673014212711484998033216133821539885421138070306477264866327549730911573831074801525177859765712567167095903919303+843401639836709482028685764607129261791330643868212867532430090507242037514006427793603581220496836139166547085499*u])
    sig E([963823355633972122114533498175662916621992470505354782789337615847591161145194281419366975300935939968232579346290+596907481049847637954275493859228934805964488037826922094320375977359016208358247522168009186501678750789366694831*u,1503040898615551538476187079486863259539849948567091887110583169943865184109068018840042625482669131770515482621711+3444166137003222945962463909857562676481832034105318967013156342862358108020440293426901361538632823324929201906078*u])
    aggPubkey E([3103244252149090420124940058491173358275189586453938010595576928631997313493844448363005953641905183987079560513835,1296246409150097609953508557969533080097715407458068120115474713311006715865163545587973784795351244083056720382121])
     */
    let assignment = PairingCircuit::<M31> {
        pubkey: [string_to_m31_array("3103244252149090420124940058491173358275189586453938010595576928631997313493844448363005953641905183987079560513835", 8), 
                string_to_m31_array("1296246409150097609953508557969533080097715407458068120115474713311006715865163545587973784795351244083056720382121", 8)],
        hm: [
            [string_to_m31_array("2128747184964102066453428909345807587167353354433686779055175069717994597853044053001604474195549116663962354781667", 8), 
            string_to_m31_array("600928199043548865756890420428378235956589666349872943435617471245143322438124492345775032317976373712791854412075", 8)], 
            [string_to_m31_array("2673014212711484998033216133821539885421138070306477264866327549730911573831074801525177859765712567167095903919303", 8),
            string_to_m31_array("843401639836709482028685764607129261791330643868212867532430090507242037514006427793603581220496836139166547085499", 8)]
            ],
        sig: [
            [string_to_m31_array("963823355633972122114533498175662916621992470505354782789337615847591161145194281419366975300935939968232579346290", 8), 
            string_to_m31_array("596907481049847637954275493859228934805964488037826922094320375977359016208358247522168009186501678750789366694831", 8),],
            [string_to_m31_array("1503040898615551538476187079486863259539849948567091887110583169943865184109068018840042625482669131770515482621711", 8),
            string_to_m31_array("3444166137003222945962463909857562676481832034105318967013156342862358108020440293426901361538632823324929201906078", 8)]
        ]
    };
	let test_time = 16;
    let mut assignments = vec![];
    for i in 0..test_time {
        assignments.push(assignment.clone());
    }

	let compile_result = compile_generic(&PairingCircuit::default(),CompileOptions::default()).unwrap();
    let start_time = std::time::Instant::now();
	let witness = compile_result
	.witness_solver
	.solve_witnesses_with_hints(&assignments, &mut hint_registry)
	.unwrap();
	let end_time = std::time::Instant::now();
	println!("Generate witness Time: {:?}", end_time.duration_since(start_time));
	run_circuit::<M31Config, M31ExtConfigSha2>(&compile_result, witness);
	let end_time = std::time::Instant::now();
	println!("Generate witness Time: {:?}", end_time.duration_since(start_time));
}