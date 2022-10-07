use da_primitives::{traits::ExtendedHeader, HeaderNumberTrait, KateCommitment};
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::{BlockHash, RawOrigin};
use hex_literal::hex;
use nomad_home::Nonces;
use sp_core::H256;
use sp_runtime::{traits::Header as _, Digest};
use sp_std::vec;

use crate::*;

benchmarks! {
	where_clause {
		where
			[u8; 32]: From<<T as frame_system::Config>::AccountId>,
			H256: From<<T as frame_system::Config>::Hash>,
			H256: Into<<T as frame_system::Config>::Hash>,
			<T as frame_system::Config>::BlockNumber: HeaderNumberTrait,
			u32: From<<T as frame_system::Config>::BlockNumber>,
			T::Header: ExtendedHeader,
			<<T as frame_system::Config>::Header as ExtendedHeader>::Root: From<KateCommitment<H256>>
			// <T as frame_system::Config>::Hash: From<KateCommitment<H256>>,
	}

	try_dispatch_data_root {
		// Create extrinsics root for block 10
		let hash :H256 = hex!("03170a2e7597b7b7e3d84c05391d139a62b157e78786d8c082f29dcf4c111314").into();
		let extrinsics_root = KateCommitment { hash, ..Default::default() };

		// Create block header for block 10
		let block_number :T::BlockNumber = 10u32.into();
		let state_root = H256::repeat_byte(2u8).into();
		let parent_hash = H256::repeat_byte(1u8).into();
		let app_data_lookup = Default::default();
		let header :T::Header = ExtendedHeader::new(
			block_number.clone(),
			extrinsics_root.into(),
			state_root,
			parent_hash,
			Digest { logs: vec![] },
			app_data_lookup);
		let header_hash :T::Hash = header.hash();

		// Insert 10th block's hash into block number --> hash mapping so
		// submitting 10th block's header is accepted by pallet
		BlockHash::<T>::insert(block_number, header_hash);

		// Get home's current merkle root pre-enqueue
		let origin = RawOrigin::Signed(whitelisted_caller::<T::AccountId>());
		let destination_domain = 1000;
		let recipient_address = H256::zero();

		let pre_nonce = Nonces::<T>::get(destination_domain);

	}: _(origin, destination_domain, recipient_address, header)
	verify {
		let post_nonce = Nonces::<T>::get(destination_domain);
		assert_eq!(pre_nonce +1, post_nonce);
	}
}
