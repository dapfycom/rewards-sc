#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait Contract {
    #[init]
    fn init(&self) {}


    #[endpoint]
    #[payable("EGLD")]
    fn supply(&self) {
        let amount = self.call_value().egld_value(); // get the amount of EGLD sent to the contract
        self.supply_amount().update(|supply_amount| *supply_amount += &*amount); // update the supply amount
    }

    #[endpoint]
    fn set_users(&self, new_board_members: &ManagedVec<ManagedAddress>) {

        let current_epoch = self.blockchain().get_block_epoch();
        let user = self.blockchain().get_caller();        
        let registered_users_mapper = self.registered_users(current_epoch);
        for addr in new_board_members {
            let _ = registered_users_mapper.insert(addr);
        }
       
    }

    #[endpoint]
    fn distribute_egld(&self) {
        let current_epoch = self.blockchain().get_block_epoch();
        let registered_users_mapper = self.registered_users(current_epoch);
        let supply_amount = self.supply_amount().get();
        let amount_to_receive = supply_amount / registered_users_mapper.len() as u64;

        for addr in registered_users_mapper.iter() {
            self.user_balance(addr).update(|balance| *balance += amount_to_receive);
        }
    }

    #[endpoint]
    fn claim(&self) {
        let user = self.blockchain().get_caller();        
        let amount_to_receive = self.user_balance(&user).get();

        self.send().direct_egld(&user, &amount_to_receive)
    }


  


    // storage
    #[view(getSupplyAmount)]
    #[storage_mapper("supply_amount")]
    fn supply_amount(&self) -> SingleValueMapper<BigUint>;
   
   

    #[storage_mapper("registered_users")]
    fn registered_users(&self, epoch: u64) -> UnorderedSetMapper<ManagedAddress>;

    #[storage_mapper("user_balance")]
    fn user_balance(&self, user:  ManagedAddress) -> SingleValueMapper<BigUint>;
}
