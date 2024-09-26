//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as PalletKitties;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

//cargo build --profile=production --features runtime-benchmarks
///
/**
./target/production/solochain-template-node benchmark pallet \--chain dev \
--execution=wasm \
--wasm-execution=compiled \
--pallet pallet_poe \
--extrinsic "*" \
--steps 20 \
--repeat 10 \
--output pallets/kitties/src/weights.rs \
--template .maintain/frame-weight-template.hbs
 *  */
///
// cargo build --profile=production
//
#[benchmarks]
mod benches {
    use super::*;

    // #[benchmark]
    // fn do_something() {
    //     let value = 100u32.into();
    //     let caller: T::AccountId = whitelisted_caller();
    //     #[extrinsic_call]
    //     do_something(RawOrigin::Signed(caller), value);

    //     assert_eq!(Something::<T>::get(), Some(value));
    // }

    // #[benchmark]
    // fn cause_error() {
    //     Something::<T>::put(100u32);
    //     let caller: T::AccountId = whitelisted_caller();
    //     #[extrinsic_call]
    //     cause_error(RawOrigin::Signed(caller));

    //     assert_eq!(Something::<T>::get(), Some(101u32));
    // }

    #[benchmark]
    fn create() -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        #[extrinsic_call]
        create(RawOrigin::Signed(caller.clone()));

        Ok(())
    }

    impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
