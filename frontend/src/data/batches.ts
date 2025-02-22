export interface Batch {
    l1StartHash: string;
    l1EndHash: string;
    l2StartHeight: number;
    l2EndHeight: number;
    status: string;
    commitment: string;
    commitmentTransactionID: string;
    proofTransactionID: string | null;
}

const batchesData: Batch[] = [
    {
        l1StartHash: "00000...d3f72",
        l1EndHash: "00000...7c4b5",
        l2StartHeight: 3740152,
        l2EndHeight: 3741251,
        status: "Committed Finalized",
        commitment: "5af3c...4e1a8",
        commitmentTransactionID: "6bcd8...90ef2",
        proofTransactionID: null
    },
    {
        l1StartHash: "00000...9d251",
        l1EndHash: "00000...d3f72",
        l2StartHeight: 3739152,
        l2EndHeight: 3740151,
        status: "Committed Finalized",
        commitment: "82db4...7fc12",
        commitmentTransactionID: "1a7e4...9d6a3",
        proofTransactionID: null
    },
    {
        l1StartHash: "00000...5ac13",
        l1EndHash: "00000...9d251",
        l2StartHeight: 3738252,
        l2EndHeight: 3739151,
        status: "Committed Finalized",
        commitment: "34c9b...cd1a2",
        commitmentTransactionID: "8e2f5...ca7b6",
        proofTransactionID: null
    },
    {
        l1StartHash: "00000...72f99",
        l1EndHash: "00000...5ac13",
        l2StartHeight: 3737252,
        l2EndHeight: 3738251,
        status: "Committed Finalized",
        commitment: "7b13e...0a4c8",
        commitmentTransactionID: "5e4b2...8c1d9",
        proofTransactionID: null
    },
    {
        l1StartHash: "00000...b245e",
        l1EndHash: "00000...72f99",
        l2StartHeight: 3736252,
        l2EndHeight: 3737251,
        status: "Committed Finalized",
        commitment: "1e7db...3c8a2",
        commitmentTransactionID: "0d8a7...bf4e9",
        proofTransactionID: null
    },
    {
        l1StartHash: "00000...3e6b1",
        l1EndHash: "00000...b245e",
        l2StartHeight: 3735252,
        l2EndHeight: 3736251,
        status: "Committed Finalized",
        commitment: "6b9cd...e4a5c",
        commitmentTransactionID: "9b7e1...2c3d4",
        proofTransactionID: null
    },
    {
        l1StartHash: "00000...cd217",
        l1EndHash: "00000...3e6b1",
        l2StartHeight: 3734252,
        l2EndHeight: 3735251,
        status: "Committed Finalized",
        commitment: "3b12f...a7e48",
        commitmentTransactionID: "7c9b8...1f2e3",
        proofTransactionID: null
    },
    {
        l1StartHash: "00000...e3a45",
        l1EndHash: "00000...cd217",
        l2StartHeight: 3733252,
        l2EndHeight: 3734251,
        status: "Committed Finalized",
        commitment: "8f2d7...9a4b3",
        commitmentTransactionID: "5d1a9...c4b27",
        proofTransactionID: null
    },
    {
        l1StartHash: "00000...c718d",
        l1EndHash: "00000...e3a45",
        l2StartHeight: 3732252,
        l2EndHeight: 3733251,
        status: "Committed Finalized",
        commitment: "0b7a3...f1e98",
        commitmentTransactionID: "4b6c9...2a71d",
        proofTransactionID: null
    },
    {
        l1StartHash: "00000...d41e2",
        l1EndHash: "00000...c718d",
        l2StartHeight: 3731252,
        l2EndHeight: 3732251,
        status: "Committed Finalized",
        commitment: "2d14b...e8c43",
        commitmentTransactionID: "3f7c8...6b9d4",
        proofTransactionID: null
    }
];

export default batchesData;