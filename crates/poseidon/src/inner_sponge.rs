use ark_ff::Field;
use halo_group::wrappers::PastaConfig;
use ark_ff::Zero;

const SPONGE_CAPACITY: usize = 1;
const SPONGE_RATE: usize = 2;
const PERM_ROUNDS_FULL: usize = 55;
const STATE_SIZE: usize = SPONGE_CAPACITY + SPONGE_RATE;

// -------------------- Helpers -------------------- //

// sbox(x) = x^7
fn sbox<F: Field>(mut x: F) -> F {
    let mut square = x;
    square.square_in_place();
    x *= square;
    square.square_in_place();
    x *= square;
    x
}

fn apply_mds_matrix<P: PastaConfig>(state: &[P::BaseField; STATE_SIZE]) -> [P::BaseField; STATE_SIZE] {
    let mut ret = [P::BaseField::zero(); 3];
    for i in 0..P::POSEIDON_MDS.len() {
        ret[i] = state.iter().zip(P::POSEIDON_MDS[i]).map(|(s, m)| *s * m).sum::<P::BaseField>();
    }
    ret
}

fn full_round<P: PastaConfig>(state: &mut [P::BaseField; STATE_SIZE], r: usize) {
    for state_i in state.iter_mut() {
        *state_i = sbox(*state_i);
    }
    *state = apply_mds_matrix::<P>(state);
    for (i, x) in P::POSEIDON_ROUND_CONSTANTS[r].iter().enumerate() {
        state[i] += x;
    }
}

#[derive(Clone, Debug)]
pub enum SpongeState {
    Absorbed(usize),
    Squeezed(usize),
}

#[derive(Clone)]
pub struct PoseidonSponge<P: PastaConfig> {
    pub sponge_state: SpongeState,
    pub state: [P::BaseField; 3],
}

impl<P: PastaConfig> PoseidonSponge<P> {
    fn poseidon_block_cipher(&mut self) {
        for r in 0..PERM_ROUNDS_FULL {
            full_round::<P>(&mut self.state, r);
        }
    }

    pub fn new() -> Self {
        Self {
            state: [<P::BaseField as Zero>::zero(); STATE_SIZE],
            sponge_state: SpongeState::Absorbed(0),
        }
    }

    pub fn absorb(&mut self, x: &[P::BaseField]) {
        for x in x.iter() {
            match self.sponge_state {
                SpongeState::Absorbed(n) => {
                    if n == SPONGE_RATE {
                        self.poseidon_block_cipher();
                        self.sponge_state = SpongeState::Absorbed(1);
                        self.state[0] += x;
                    } else {
                        self.sponge_state = SpongeState::Absorbed(n + 1);
                        self.state[n] += x;
                    }
                }
                SpongeState::Squeezed(_n) => {
                    self.state[0] += x;
                    self.sponge_state = SpongeState::Absorbed(1);
                }
            }
        }
    }

    pub fn squeeze(&mut self) -> P::BaseField {
        match self.sponge_state {
            SpongeState::Squeezed(n) => {
                if n == SPONGE_RATE {
                    self.poseidon_block_cipher();
                    self.sponge_state = SpongeState::Squeezed(1);
                    self.state[0]
                } else {
                    self.sponge_state = SpongeState::Squeezed(n + 1);
                    self.state[n]
                }
            }
            SpongeState::Absorbed(_n) => {
                self.poseidon_block_cipher();
                self.sponge_state = SpongeState::Squeezed(1);
                self.state[0]
            }
        }
    }

    pub fn reset(&mut self) {
        self.state = [P::BaseField::zero(); STATE_SIZE];
        self.sponge_state = SpongeState::Absorbed(0);
    }
}

#[cfg(test)]
mod self_tests {
    use super::*;
    use ark_ff::{AdditiveGroup, Field};
    use ark_pallas::PallasConfig;
    use ark_vesta::Fr as Fp;

    fn minas_apply_mds_matrix(state: &[Fp]) -> Vec<Fp> {
        PallasConfig::POSEIDON_MDS
            .iter()
            .map(|m| {
                state
                    .iter()
                    .zip(m.iter())
                    .fold(Fp::ZERO, |x, (s, &m)| m * s + x)
            })
            .collect()
    }

    // Helper to create Fp elements from integers for testing
    fn fp_from_u64(x: u64) -> Fp {
        Fp::from(x)
    }

    #[test]
    fn test_sbox() {
        // Test sbox(x) = x^7
        let x = fp_from_u64(2);
        let expected = x.pow([7]);
        assert_eq!(sbox(x), expected, "sbox(2) should compute 2^7");

        // Test sbox(0) = 0
        assert_eq!(sbox(Fp::ZERO), Fp::ZERO, "sbox(0) should be 0");

        // Test sbox(1) = 1
        assert_eq!(sbox(Fp::ONE), Fp::ONE, "sbox(1) should be 1");
    }

    #[test]
    fn test_apply_mds_matrix() {
        let state = [fp_from_u64(1), fp_from_u64(2), fp_from_u64(3)];
        let result = minas_apply_mds_matrix(&state);
        assert_eq!(
            result.len(),
            STATE_SIZE,
            "MDS output length should be STATE_SIZE"
        );

        // Assuming FP_MDS is a 3x3 matrix, verify against expected output
        // Replace with actual expected values based on FP_MDS
        let expected = apply_mds_matrix::<PallasConfig>(&state).to_vec();
        assert_eq!(
            result, expected,
            "apply_mds_matrix should match apply_mds_matrix_2"
        );
    }

    #[test]
    fn test_poseidon_sponge_new() {
        let sponge = PoseidonSponge::<PallasConfig>::new();
        assert_eq!(
            sponge.state,
            [Fp::ZERO; 3],
            "New sponge state should be zero"
        );
        assert!(
            matches!(sponge.sponge_state, SpongeState::Absorbed(0)),
            "New sponge should be in Absorbed(0) state"
        );
    }

    #[test]
    fn test_poseidon_sponge_absorb_single() {
        let mut sponge = PoseidonSponge::<PallasConfig>::new();
        let input = fp_from_u64(42);
        sponge.absorb(&[input]);
        assert_eq!(
            sponge.state[0], input,
            "First absorb should add input to state[0]"
        );
        assert_eq!(sponge.state[1], Fp::ZERO, "state[1] should remain zero");
        assert_eq!(sponge.state[2], Fp::ZERO, "state[2] should remain zero");
        assert!(
            matches!(sponge.sponge_state, SpongeState::Absorbed(1)),
            "Sponge state should be Absorbed(1)"
        );
    }

    #[test]
    fn test_poseidon_sponge_reset() {
        let mut sponge = PoseidonSponge::<PallasConfig>::new();
        sponge.absorb(&[fp_from_u64(42)]);
        sponge.squeeze();
        sponge.reset();
        assert_eq!(sponge.state, [Fp::ZERO; 3], "Reset should clear state");
        assert!(
            matches!(sponge.sponge_state, SpongeState::Absorbed(0)),
            "Reset should set state to Absorbed(0)"
        );
    }
}

#[cfg(test)]
mod mina_tests {
    use super::*;
    use ark_ff::Field;
    use ark_pallas::PallasConfig;
    use ark_vesta::Fq;
    use ark_vesta::Fr as Fp;
    use ark_vesta::VestaConfig;
    use serde::Deserialize;
    use std::{fs::File, path::PathBuf}; // needed for ::new() sponge

    //
    // Helpers for test vectors
    //

    #[derive(Debug, Deserialize)]
    struct TestVectors {
        test_vectors: Vec<TestVector>,
    }

    #[derive(Debug, Deserialize)]
    struct TestVector {
        input: Vec<String>,
        output: String,
    }

    fn from_hex<F: Field>(hex: &str) -> F {
        let bytes: Vec<u8> = hex::decode(hex).unwrap();
        F::deserialize_uncompressed(&mut &bytes[..]).unwrap()
    }

    fn test_vectors<F>(hash: F)
    where
        F: Fn(&[Fp]) -> Fp,
    {
        // read test vectors from given file
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("test-vectors");
        path.push("kimchi-vecs.json");
        let file = File::open(&path).expect("couldn't open test vector file");
        let test_vectors: TestVectors =
            serde_json::from_reader(file).expect("couldn't deserialize test vector file");

        // execute test vectors
        for test_vector in test_vectors.test_vectors {
            // deserialize input & ouptut
            let input: Vec<Fp> = test_vector
                .input
                .into_iter()
                .map(|hexstring| from_hex(&hexstring))
                .collect();
            let expected_output = from_hex(&test_vector.output);

            // hash & check against expect output
            assert_eq!(hash(&input), expected_output);
        }
    }

    //
    // Tests
    //

    #[test]
    fn poseidon_test_vectors_kimchi() {
        fn hash(input: &[Fp]) -> Fp {
            let mut hash = PoseidonSponge::<PallasConfig>::new();
            hash.absorb(input);
            hash.squeeze()
        }
        test_vectors(hash);
    }

    #[test]
    fn manual_mina_fq() {
        let expected_out_hex = "967b83532be4e2aa3c9fba40f38413dd7474eab2730a821327d9b9c578a75401";
        let inputs_hex = [
            "289fc11cca6044b7bdcc8262d3940f58f802e35e4f8e51131468273ee55ec50f",
            "e3af7fad787d37e13c0a1273fb24840c95127bac6dc41254ebd66d35db4d3911",
            "46896544d2fa3b790493ec3d90f144ab766b5b9c3c540b5aea8ec62067c00134",
            "bf55053f47e2ae9086c26ce65ab000394aa798edc4c37ce273b01bf7b4494616",
            "c37234e114e98a50c2ea8256cb3db9673e3da2786816782c646c7a5ff4d9841f",
            "221f730ada69e849c7bb696bc95b531fa9e443ca7c5f2f7ea3a33a525637c12e",
            "ace1b4e8db4e4cb5684b333825b8cccb2ca371b53695cf1aa4d62c7464273836",
            "43ebe40987d6f599966641d3c2c91bc468c3c2bcba39589f67e1b5dee9946f21",
            "a0e120ac5159f96e10bf7b53a2e5f64dba0b6f7a6af98c9aa9b60d6383413410",
            "ece80fe77b11ca43fc49588ffca09e7409896723f18a1859060fda7c460dde0d",
        ];

        let mut sponge = PoseidonSponge::<VestaConfig>::new();
        let inputs_fq: Vec<_> = inputs_hex.into_iter().map(|x| from_hex::<Fq>(x)).collect();
        sponge.absorb(&inputs_fq);

        assert_eq!(sponge.squeeze(), from_hex(expected_out_hex))
    }

    #[test]
    fn manual_mina_fp() {
        let expected_out_hex = "e6d13dd2829ea988129712bd474623148048e92795dc21aa4c20d14c77c8c224";
        let inputs_hex = [
            "215d22e6b8b29353cc85d50db2f71a2cb192d224237f474df0a826b2fa2eda27",
            "243d58bd94dd62fad76516156b101aa833623e5a7b637120e0da63dcc800c72c",
            "7713135c3832640b32dff361b734e8316eb90afbadd465dd64b2ae5e523ad011",
            "a7ba00a7d67b005291d9b113cceadbf29a907eb3a14777b2a473ccbe86bb8303",
            "2b5247a6ce29aa0c8d447c64c8258c9259f1e5ef2dda92a32f0217a822e7ec18",
            "d8e4773e65b0f16973a9512dabd5275a1760315c7a67003eef734789a9c3c218",
            "a9523942b73756595e693b120c645d97075af5bf6cbc4f85bbd31ba160183616",
            "103eb4443a7b5065f6e58cf2a671f30b3073c25a73d861927c5916ab62b7ad17",
            "2f6efa8e63a7c84ef88945f5dc97be35c9c3ab82ea1837253192e86e9d512429",
            "1afdc9e9dd0adfc9130e22f03191916dbd0f51b304d2d1ecc395a952c4b13b24",
        ];

        let mut sponge = PoseidonSponge::<PallasConfig>::new();
        let inputs_fp: Vec<_> = inputs_hex.into_iter().map(|x| from_hex::<Fp>(x)).collect();
        sponge.absorb(&inputs_fp);

        assert_eq!(sponge.squeeze(), from_hex(expected_out_hex))
    }

    // #[test]
    // fn test_regression_challenge_empty_vesta_kimchi() {
    //     let mut sponge = PoseidonSponge::<Fq>::new();
    //     let output = sponge.squeeze();
    //     let exp_output =
    //         from_hex("c1e504c0184cce70a605d2f942d579c500000000000000000000000000000000").unwrap();
    //     assert_eq!(output, exp_output);
    // }

    // #[test]
    // fn test_regression_challenge_empty_pallas_kimchi() {
    //     let mut sponge = PoseidonSponge::<Fp>::new();
    //     let output = sponge.squeeze();
    //     let exp_output =
    //         from_hex("a8eb9ee0f30046308abbfa5d20af73c800000000000000000000000000000000").unwrap();
    //     assert_eq!(output, exp_output);
    // }

    // #[test]
    // fn test_poseidon_vesta_kimchi_challenge_is_squeezed_to_128_bits() {
    //     // Test that the challenge is less than 2^128, i.e. the sponge state is
    //     // squeezed to 128 bits
    //     let mut sponge = DefaultFqSponge::<VestaParameters, PlonkSpongeConstantsKimchi>::new(
    //         fq_kimchi::static_params(),
    //     );
    //     let mut rng = o1_utils::tests::make_test_rng(None);
    //     let random_n = rng.gen_range(1..50);
    //     let random_fq_vec = (0..random_n)
    //         .map(|_| Fq::rand(&mut rng))
    //         .collect::<Vec<Fq>>();
    //     sponge.absorb_fq(&random_fq_vec);
    //     let challenge = sponge.challenge();
    //     let two_128 = Fp::from(2).pow([128]);
    //     assert!(challenge < two_128);
    // }

    // #[test]
    // fn test_poseidon_pallas_kimchi_challenge_is_squeezed_to_128_bits() {
    //     // Test that the challenge is less than 2^128, i.e. the sponge state is
    //     // squeezed to 128 bits
    //     let mut sponge = DefaultFqSponge::<PallasParameters, PlonkSpongeConstantsKimchi>::new(
    //         fp_kimchi::static_params(),
    //     );
    //     let mut rng = o1_utils::tests::make_test_rng(None);
    //     let random_n = rng.gen_range(1..50);
    //     let random_fp_vec = (0..random_n)
    //         .map(|_| Fp::rand(&mut rng))
    //         .collect::<Vec<Fp>>();
    //     sponge.absorb_fq(&random_fp_vec);
    //     let challenge = sponge.challenge();
    //     let two_128 = Fq::from(2).pow([128]);
    //     assert!(challenge < two_128);
    // }

    // #[test]
    // fn test_poseidon_pallas_absorb_point_to_infinity() {
    //     let mut sponge = DefaultFqSponge::<PallasParameters, PlonkSpongeConstantsKimchi>::new(
    //         fp_kimchi::static_params(),
    //     );
    //     let point = Pallas::zero();
    //     sponge.absorb_g(&[point]);
    //     let exp_output = [Fp::from(0); 3];
    //     assert_eq!(sponge.sponge.state, exp_output);
    // }

    // #[test]
    // fn test_poseidon_vesta_absorb_point_to_infinity() {
    //     let mut sponge = DefaultFqSponge::<VestaParameters, PlonkSpongeConstantsKimchi>::new(
    //         fq_kimchi::static_params(),
    //     );
    //     let point = Vesta::zero();
    //     sponge.absorb_g(&[point]);
    //     let exp_output = [Fq::from(0); 3];
    //     assert_eq!(sponge.sponge.state, exp_output);
    // }

    // #[test]
    // fn test_poseidon_challenge_multiple_times_without_absorbtion() {
    //     let mut sponge = DefaultFqSponge::<VestaParameters, PlonkSpongeConstantsKimchi>::new(
    //         fq_kimchi::static_params(),
    //     );
    //     let mut rng = o1_utils::tests::make_test_rng(None);
    //     let random_n = rng.gen_range(10..50);

    //     let mut old_state = sponge.sponge.state.clone();
    //     let mut new_state = sponge.sponge.state.clone();
    //     // Only to avoid a warning. old_state must be used.
    //     assert_eq!(
    //         old_state, new_state,
    //         "States must be the same after initialization"
    //     );
    //     let mut challenges: Vec<_> = vec![];

    //     for i in 0..random_n {
    //         old_state.clone_from(&new_state);
    //         new_state.clone_from(&sponge.sponge.state);
    //         let chal = sponge.challenge();
    //         if i % 2 == 0 {
    //             assert_eq!(
    //                 old_state, new_state,
    //                 "States must be the same after squeezing an even number of times"
    //             );
    //         } else {
    //             assert_ne!(
    //                 old_state, new_state,
    //                 "States must not be the same after squeezing an odd number of times"
    //             );
    //         }
    //         assert!(
    //             !challenges.contains(&chal),
    //             "Challenges must always be different, even without any absorbtion"
    //         );
    //         challenges.push(chal);
    //     }
    // }
}
