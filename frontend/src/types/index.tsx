export interface RpcCheckpointInfoBatchExp {
    idx: number;               // Index of the checkpoint
    l1_range: [number, number]; // L1 height range (start, end)
    l2_range: [number, number]; // L2 height range (start, end)
    l2_blockid: string;        // L2 block ID
    commitment?: RpcCheckpointCommitmentInfo;
    confirmation_status?: string;
}

export interface PaginatedData<T> {
    current_page: number,
    total_pages: number,
    absolute_first_page: number, // Will be 0 or 1, depending on the context
    items: T[],            // The items for the current page
}


export interface RpcCheckpointCommitmentInfo {
    blockhash: string;
    // for batch explorer `txid` is the only thing we care about.
    txid: string
    wtxid: string
    height: number,
    position: number,
}