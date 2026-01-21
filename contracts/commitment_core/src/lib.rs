#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec, Map, i128, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommitmentRules {
    pub duration_days: u32,
    pub max_loss_percent: u32,
    pub commitment_type: String, // "safe", "balanced", "aggressive"
    pub early_exit_penalty: u32,
    pub min_fee_threshold: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Commitment {
    pub commitment_id: String,
    pub owner: Address,
    pub nft_token_id: u32,
    pub rules: CommitmentRules,
    pub amount: i128,
    pub asset_address: Address,
    pub created_at: u64,
    pub expires_at: u64,
    pub current_value: i128,
    pub status: String, // "active", "settled", "violated", "early_exit"
}

#[contract]
pub struct CommitmentCoreContract;

#[contractimpl]
impl CommitmentCoreContract {
    /// Initialize the core commitment contract
    pub fn initialize(e: Env, admin: Address, nft_contract: Address) {
        // TODO: Store admin and NFT contract address
        // TODO: Initialize storage
    }

    /// Create a new commitment
    pub fn create_commitment(
        e: Env,
        owner: Address,
        amount: i128,
        asset_address: Address,
        rules: CommitmentRules,
    ) -> String {
        // TODO: Validate rules
        // TODO: Transfer assets from owner to contract
        // TODO: Call NFT contract to mint Commitment NFT
        // TODO: Store commitment data
        // TODO: Emit creation event
        String::from_str(&e, "commitment_id_placeholder")
    }

    /// Get commitment details
    pub fn get_commitment(e: Env, commitment_id: String) -> Commitment {
        // TODO: Retrieve commitment from storage
        Commitment {
            commitment_id: String::from_str(&e, "placeholder"),
            owner: Address::from_string(&String::from_str(&e, "placeholder")),
            nft_token_id: 0,
            rules: CommitmentRules {
                duration_days: 0,
                max_loss_percent: 0,
                commitment_type: String::from_str(&e, "placeholder"),
                early_exit_penalty: 0,
                min_fee_threshold: 0,
            },
            amount: 0,
            asset_address: Address::from_string(&String::from_str(&e, "placeholder")),
            created_at: 0,
            expires_at: 0,
            current_value: 0,
            status: String::from_str(&e, "active"),
        }
    }

    /// Update commitment value (called by allocation logic)
    pub fn update_value(e: Env, commitment_id: String, new_value: i128) {
        // TODO: Verify caller is authorized (allocation contract)
        // TODO: Update current_value
        // TODO: Check if max_loss_percent is violated
        // TODO: Emit value update event
    }

    /// Check if commitment rules are violated
    pub fn check_violations(e: Env, commitment_id: String) -> bool {
        // TODO: Check if max_loss_percent exceeded
        // TODO: Check if duration expired
        // TODO: Check other rule violations
        false
    }

    /// Settle commitment at maturity
    pub fn settle(e: Env, commitment_id: String) {
        // Storage key for commitments: ("commitments", commitment_id)
        let key = (Symbol::short("commitments"), commitment_id.clone());

        // Retrieve commitment
        let maybe: Option<Commitment> = e.storage().get(&key);
        let mut commitment = match maybe {
            Some(c) => c,
            None => panic!("Commitment not found"),
        };

        // Check already settled
        if commitment.status == String::from_str(&e, "settled") {
            panic!("Commitment already settled");
        }

        // Current ledger timestamp
        let now: u64 = e.ledger().timestamp();

        // Check expiration
        if now < commitment.expires_at {
            panic!("Commitment not expired");
        }

        // Calculate settlement amount: prefer current_value when non-zero
        let settlement_amount: i128 = if commitment.current_value != 0 {
            commitment.current_value
        } else {
            commitment.amount
        };

        // Attempt asset transfer from this contract to the owner.
        // We call the asset contract `transfer` entrypoint with (from, to, amount).
        // The `from` address is the contract's own address.
        let from = Address::Contract(e.get_current_contract());
        let to = commitment.owner.clone();

        // Invoke token transfer; errors will cause a trap which bubbles up as test failure.
        // Some token contracts expect different signatures; adapt if needed.
        let _ = e.invoke_contract(
            &commitment.asset_address,
            &Symbol::short("transfer"),
            (from.clone(), to.clone(), settlement_amount),
        );

        // Mark commitment as settled and update storage
        commitment.status = String::from_str(&e, "settled");
        e.storage().set(&key, &commitment);

        // Call NFT contract to mark NFT as settled (if available)
        // The NFT contract is expected to have a `settle` method accepting token_id: u32
        let _ = e.invoke_contract(
            &e.storage().get(&Symbol::short("nft_contract")).unwrap_or(Address::from_string(&String::from_str(&e, ""))),
            &Symbol::short("settle"),
            (commitment.nft_token_id,),
        );

        // Emit event: CommitmentSettled(commitment_id, owner, settlement_amount, timestamp)
        e.events().publish((Symbol::short("CommitmentSettled"),), (commitment_id, to, settlement_amount, now));
    }

    /// Early exit (with penalty)
    pub fn early_exit(e: Env, commitment_id: String, caller: Address) {
        // TODO: Verify caller is owner
        // TODO: Calculate penalty
        // TODO: Transfer remaining amount (after penalty) to owner
        // TODO: Mark commitment as early_exit
        // TODO: Emit early exit event
    }

    /// Allocate liquidity (called by allocation strategy)
    pub fn allocate(e: Env, commitment_id: String, target_pool: Address, amount: i128) {
        // TODO: Verify caller is authorized allocation contract
        // TODO: Verify commitment is active
        // TODO: Transfer assets to target pool
        // TODO: Record allocation
        // TODO: Emit allocation event
    }
}

