mod circuits;

use circuits::TestCircuit;
use std::marker::PhantomData;

use halo2_proofs::dev::MockProver;

fn main() {
    use halo2_proofs::halo2curves::bn256::Fr;

    let circuit = TestCircuit::<Fr> { _ph: PhantomData };
    let prover = MockProver::run(8, &circuit, vec![]).unwrap();
    prover.verify().unwrap();
}
