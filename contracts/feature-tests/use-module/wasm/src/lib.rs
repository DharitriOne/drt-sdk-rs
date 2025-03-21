// Code generated by the drt-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           63
// Async Callback:                       1
// Total number of exported functions:  65

#![no_std]

drt_sc_wasm_adapter::allocator!();
drt_sc_wasm_adapter::panic_handler!();

drt_sc_wasm_adapter::endpoints! {
    use_module
    (
        init => constructor_in_a_module
        checkFeatureGuard => check_feature_guard
        checkPause => check_pause
        call_contract_base_full_path_endpoint => call_contract_base_full_path_endpoint
        call_contract_base_endpoint => call_contract_base_endpoint
        call_mod_a => call_mod_a
        call_mod_b => call_mod_b
        call_mod_c => call_mod_c
        only_owner_mod_endpoint => only_owner_mod_endpoint
        call_derived_not_owner_only => call_derived_not_owner_only
        only_admin_mod_endpoint => only_admin_mod_endpoint
        call_derived_not_admin_only => call_derived_not_admin_only
        countTo100 => count_to_100
        mergeTokens => merge_tokens_endpoint
        mergeTokensCustomAttributes => merge_tokens_custom_attributes_endpoint
        splitTokens => split_tokens_endpoint
        splitTokenPartial => split_token_partial_endpoint
        claimDeveloperRewards => claim_developer_rewards
        dnsRegister => dns_register
        issueToken => issue_token
        setFeatureFlag => set_feature_flag_endpoint
        depositTokensForProposal => deposit_tokens_for_proposal
        withdrawGovernanceTokens => claim_deposited_tokens
        propose => propose
        vote => vote
        queue => queue
        execute => execute
        cancel => cancel
        getProposalStatus => get_proposal_status
        getProposer => get_proposer
        getProposalDescription => get_proposal_description
        getProposalActions => get_proposal_actions
        getProposalVotes => proposal_votes
        getTotalVotes => total_votes
        getTotalDownvotes => total_downvotes
        changeQuorum => change_quorum
        changeMinTokenBalanceForProposing => change_min_token_balance_for_proposing
        changeVotingDelayInBlocks => change_voting_delay_in_blocks
        changeVotingPeriodInBlocks => change_voting_period_in_blocks
        changeLockTimeAfterVotingEndsInBlocks => change_lock_time_after_voting_ends_in_blocks
        getGovernanceTokenId => governance_token_id
        getQuorum => quorum
        getMinFeeForPropose => min_fee_for_propose
        getMinTokenBalanceForProposing => min_token_balance_for_proposing
        getVotingDelayInBlocks => voting_delay_in_blocks
        getVotingPeriodInBlocks => voting_period_in_blocks
        getLockTimeAfterVotingEndsInBlocks => lock_time_after_voting_ends_in_blocks
        pause => pause_endpoint
        unpause => unpause_endpoint
        isPaused => paused_status
        stake => stake
        unstake => unstake
        voteSlashMember => vote_slash_member
        cancelVoteSlashMember => cancel_vote_slash_member
        slashMember => slash_member
        issueMergedToken => issue_merged_token
        addMergeableTokensToWhitelist => add_mergeable_tokens_to_whitelist
        removeMergeableTokensFromWhitelist => remove_mergeable_tokens_from_whitelist
        getMergedTokenId => merged_token
        getMergeableTokensWhitelist => mergeable_tokens_whitelist
        isAdmin => is_admin
        addAdmin => add_admin
        removeAdmin => remove_admin
        getAdmins => admins
    )
}

drt_sc_wasm_adapter::async_callback! { use_module }
