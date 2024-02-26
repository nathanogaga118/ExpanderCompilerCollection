package main

import (
	"os"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/std/hash/mimc"

	gkr "github.com/Zklib/gkr-compiler"
	"github.com/Zklib/gkr-compiler/test"
)

const NHashes = 100

type Circuit struct {
	PreImage [NHashes]frontend.Variable
	Hash     [NHashes]frontend.Variable
}

func mimcHash(api frontend.API, preImage frontend.Variable) frontend.Variable {
	mimc, _ := mimc.NewMiMC(api)
	mimc.Write(preImage)
	return mimc.Sum()
}

// Define declares the circuit's constraints
func (circuit *Circuit) Define(api frontend.API) error {
	for i := 0; i < NHashes; i++ {
		t := api.(gkr.API).MemorizedCall(mimcHash, circuit.PreImage[i])
		api.AssertIsEqual(circuit.Hash[i], t)
	}
	return nil
}

func main() {
	circuit, err := gkr.Compile(ecc.BN254.ScalarField(), &Circuit{})
	if err != nil {
		panic(err)
	}

	c := circuit.GetLayeredCircuit()
	os.WriteFile("circuit.txt", c.Serialize(), 0o644)

	assignment := &Circuit{}
	for i := 0; i < NHashes; i++ {
		assignment.PreImage[i] = "16130099170765464552823636852555369511329944820189892919423002775646948828469"
		assignment.Hash[i] = "12886436712380113721405259596386800092738845035233065858332878701083870690753"
	}
	witness := circuit.GetWitness(assignment)

	if !test.CheckCircuit(c, witness) {
		panic("error")
	}
}
