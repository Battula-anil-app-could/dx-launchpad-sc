// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           32
// Async Callback (empty):               1
// Total number of exported functions:  34

#![no_std]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    launchpad_locked_tokens
    (
        addTickets
        depositLaunchpadTokens
        claimLaunchpadTokens
        claimTicketPayment
        addUsersToBlacklist
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
