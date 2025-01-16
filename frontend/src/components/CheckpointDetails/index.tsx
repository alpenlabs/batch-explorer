import { useEffect, useState } from "react";
import { useLocation } from "react-router-dom";
import styles from "../../styles/App.module.css";
import { RpcCheckpointInfo } from "../../types";

const CheckpointDetails = () => {
    const location = useLocation();
    const params = new URLSearchParams(location.search);
    const page = params.get("p"); // Get the "p" query parameter
    const [checkpoint, setCheckpoint] = useState<RpcCheckpointInfo | null>(null);

    useEffect(() => {
        const fetchCheckpoint = async () => {
            const response = await fetch(`http://localhost:3000/api/checkpoint?p=${page}&ps=1`);
            const result = await response.json();

            if (result.result.items.length > 0) {
                setCheckpoint(result.result.items[0]);
            } else {
                setCheckpoint(null); // Handle no data case
            }
        };

        fetchCheckpoint();
    }, [page]);

    if (!checkpoint) {
        return <div>No checkpoint data available</div>;
    }
    return (
        <div className={styles.checkpointContainer}>
            <div className={styles.checkpointRow}>
                <span className={styles.checkpointLabel}>Batch TXID:</span>
                <span className={styles.checkpointValue}>{checkpoint.batch_txid}</span>
            </div>
            <div className={styles.checkpointRow}>
                <span className={styles.checkpointLabel}>Epoch index:</span>
                <span className={styles.checkpointValue}>{checkpoint.idx}</span>
            </div>
            <div className={styles.checkpointRow}>
                <span className={styles.checkpointLabel}>Status:</span>
                <span className={styles.checkpointValue}>{checkpoint.status}</span>
            </div>
            <div className={styles.checkpointRow}>
                <span className={styles.checkpointLabel}>Signet start block:</span>
                <span className={styles.checkpointValue}>
                    <a
                        href={`https://mempool0713bb23.devnet-annapurna.stratabtc.org/block/${checkpoint.l1_range[0]}`}
                        target="_blank"
                        rel="noreferrer"
                    >
                        {checkpoint.l1_range[0]}
                    </a>
                </span>
            </div>
            <div className={styles.checkpointRow}>
                <span className={styles.checkpointLabel}>Signet end block:</span>
                <span className={styles.checkpointValue}>
                    <a
                        href={`https://mempool0713bb23.devnet-annapurna.stratabtc.org/block/${checkpoint.l1_range[1]}`}
                        target="_blank"
                        rel="noreferrer"
                    >
                        {checkpoint.l1_range[1]}
                    </a>
                </span>
            </div>
            <div className={styles.checkpointRow}>
                <span className={styles.checkpointLabel}>Strata start block:</span>
                <span className={styles.checkpointValue}>
                    <a
                        href={`https://blockscoutb86fae58ae.devnet-annapurna.stratabtc.org/block/${checkpoint.l2_range[0]}`}
                        target="_blank"
                        rel="noreferrer"
                    >
                        {checkpoint.l2_range[0]}
                    </a>
                </span>
            </div>
            <div className={styles.checkpointRow}>
                <span className={styles.checkpointLabel}>Strata end block:</span>
                <span className={styles.checkpointValue}>
                    <a
                        href={`https://blockscoutb86fae58ae.devnet-annapurna.stratabtc.org/block/${checkpoint.l2_range[1]}`}
                        target="_blank"
                        rel="noreferrer"
                    >
                        {checkpoint.l2_range[1]}
                    </a>
                </span>
            </div>
        </div>
    );
};

export default CheckpointDetails;