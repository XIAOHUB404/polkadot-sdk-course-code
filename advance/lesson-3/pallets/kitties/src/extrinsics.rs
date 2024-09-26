use frame_support::pallet_macros::pallet_section;

/// Define all extrinsics for the pallet.
#[pallet_section]
mod dispatches {
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn create(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let value = Self::random_value(&who);

            //create new kitty
            let kitty_id = NextKittyId::<T>::get();
            Kitties::<T>::insert(kitty_id, Kitty(value));

            let next_id = kitty_id.checked_add(1).ok_or(Error::<T>::KittyIdOverflow)?;
            NextKittyId::<T>::put(next_id);

            KittyOwner::<T>::insert(kitty_id, who.clone());

            Self::deposit_event(Event::KittyCreated {
                creator: who,
                index: kitty_id,
                data: value,
            });

            Ok(())
        }
        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn breed(origin: OriginFor<T>, kitty_1: u64, kitty_2: u64) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(kitty_1 != kitty_2, Error::<T>::SameKittyId);

            ensure!(
                KittySale::<T>::contains_key(kitty_1) == false,
                Error::<T>::KittyListedForSale
            );

            ensure!(
                KittySale::<T>::contains_key(kitty_2) == false,
                Error::<T>::KittyListedForSale
            );

            let k1value = Kitties::<T>::get(kitty_1).ok_or(Error::<T>::KittyNotFound)?;
            let k2value = Kitties::<T>::get(kitty_2).ok_or(Error::<T>::KittyNotFound)?;

            let kitty_1_owner = KittyOwner::<T>::get(kitty_1).ok_or(Error::<T>::InvalidKittyId)?;
            let kitty_2_owner = KittyOwner::<T>::get(kitty_2).ok_or(Error::<T>::InvalidKittyId)?;
            ensure!(who == kitty_1_owner, Error::<T>::NotOwner);
            ensure!(who == kitty_2_owner, Error::<T>::NotOwner);
            let data = Self::breed_kitty(&who, k1value.0, k2value.0);

            //create new kitty
            let kitty_id = NextKittyId::<T>::get();
            Kitties::<T>::insert(kitty_id, Kitty(data));
            let next_id = kitty_id.checked_add(1).ok_or(Error::<T>::KittyIdOverflow)?;
            NextKittyId::<T>::put(next_id);
            KittyOwner::<T>::insert(kitty_id, who.clone());

            Self::deposit_event(Event::KittyBred {
                creator: who,
                index: kitty_id,
                data,
            });
            Ok(())
        }
        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        pub fn transfer(origin: OriginFor<T>, kitty_id: u64, to: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(
                KittySale::<T>::contains_key(kitty_id) == false,
                Error::<T>::KittyListedForSale
            );

            let owner = KittyOwner::<T>::get(kitty_id).ok_or(Error::<T>::KittyNotFound)?;
            ensure!(who == owner, Error::<T>::NotOwner);

            KittyOwner::<T>::insert(kitty_id, to.clone());

            Self::deposit_event(Event::KittyTransferred {
                id: kitty_id,
                from: who,
                to,
            });
            Ok(())
        }
        #[pallet::call_index(3)]
        #[pallet::weight(0)]
        pub fn sale(
            origin: OriginFor<T>,
            kitty_id: u64,
            until_block: BlockNumberFor<T>,
            minimum_bid_price: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(
                KittySale::<T>::contains_key(kitty_id) == false,
                Error::<T>::KittyListedForSale
            );

            let owner = KittyOwner::<T>::get(kitty_id).ok_or(Error::<T>::KittyNotFound)?;
            ensure!(who == owner, Error::<T>::NotOwner);

            let current_block = frame_system::Pallet::<T>::block_number();
            ensure!(until_block > current_block, Error::<T>::InvalidBlockNumber);

            KittySale::<T>::insert(kitty_id, (who.clone(), until_block, minimum_bid_price));

            Self::deposit_event(Event::KittySale {
                owner: who,
                kitty_id,
                until_block,
            });
            Ok(())
        }
        #[pallet::call_index(4)]
        #[pallet::weight(0)]
        pub fn bid(origin: OriginFor<T>, kitty_id: u64, price: BalanceOf<T>) -> DispatchResult {
            let bidder = ensure_signed(origin)?;

            // 获取用户的可用余额
            let free_balance = T::Currency::free_balance(&bidder);
            ensure!(&free_balance >= &price, Error::<T>::InsufficientBalance);

            ensure!(
                KittySale::<T>::contains_key(kitty_id) == true,
                Error::<T>::KittyNotFound
            );
            let (owner, until_block, minimum_bid_price) =
                KittySale::<T>::get(kitty_id).ok_or(Error::<T>::KittyNotForSale)?;

            ensure!(owner != bidder, Error::<T>::OwnerCannotBid);

            let current_block = frame_system::Pallet::<T>::block_number();
            ensure!(current_block <= until_block, Error::<T>::SaleExpired);

            ensure!(price > minimum_bid_price, Error::<T>::InvalidBidPrice);

            let bid_win = KittyWinner::<T>::get(kitty_id);
            if let Some((win_bidder, last_bid_price)) = bid_win {
                ensure!(bidder != win_bidder, Error::<T>::AlreadyBidded);
                ensure!(price > last_bid_price, Error::<T>::InvalidBidPrice);
            }

            // 尝试冻结资金
            ensure!(
                T::Currency::reserve(&bidder, price).is_ok(),
                Error::<T>::InsufficientBalance
            );

            //handle bid
            let mut bids = KittiesBid::<T>::get(kitty_id).unwrap_or_default();
            // 检查容量
            // ensure!(bids.len() < bids.capacity(), Error::<T>::BidsLimitMax);
            let new_bid = (bidder.clone(), price);

            // bids.push(new_bid.clone());

            KittiesBid::<T>::insert(kitty_id, bids);
            KittyWinner::<T>::insert(kitty_id, new_bid.clone());

            Self::deposit_event(Event::KittyBid {
                bidder,
                kitty_id,
                price,
            });

            Ok(())
        }
    }
}
