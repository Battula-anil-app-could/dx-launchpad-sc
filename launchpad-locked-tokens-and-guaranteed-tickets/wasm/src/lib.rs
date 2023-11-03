// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           33
// Async Callback (empty):               1
// Total number of exported functions:  35

#![no_std]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    launchpad_locked_tokens_and_guaranteed_tickets
    (
        addTickets
        depositLaunchpadTokens
        addUsersToBlacklist
        distributeGuaranteedTickets
        claimLaunchpadTokens
        claimTicketPayment
        getLaunchStageFlags
        getConfiguration
        getLaunchpadTokenId
        getLaunchpadTokensPerWinningTicket
        getTicketPrice
        getNumberOfWinningTickets
        setTicketPrice
        setLaunchpadTokensPerWinningTicket
        setConfirmationPeriodStartBlock
        setWinnerSelectionStartBlock
        setClaimStartBlock
        getTicketRangeForAddress
        getTotalNumberOfTicketsForAddress
        getTotalNumberOfTickets
        getNumberOfConfirmedTicketsForAddress
        filterTickets
        selectWinners
        getNumberOfWinningTicketsForAddress
        getWinningTicketIdsForAddress
        setSupportAddress
        getSupportAddress
        removeUsersFromBlacklist
        isUserBlacklisted
        confirmTickets
        hasUserClaimedTokens
        getLaunchpadTokensLockPercentage
        getLaunchpadTokensUnlockEpoch
    )
}

dharitri_sc_wasm_adapter::empty_callback! {}
