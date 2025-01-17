import { useEffect, useState } from "react";
import { useLocation } from "react-router-dom";
import styles from "../../styles/CheckpointDetails.module.css";
import { RpcCheckpointInfo } from "../../types";
import Pagination from "../Paginator/Pagination";
const CheckpointDetails = () => {
    const location = useLocation();
    const params = new URLSearchParams(location.search);
    const page = params.get("p"); // Get the "p" query parameter

    // if page is not a number, set it to 0
    const [checkpoint, setData] = useState<RpcCheckpointInfo | null>(null);
    const [currentPage, setCurrentPage] = useState<number>(isNaN(Number(page)) ? 0 : Number(page));
    const [rowsPerPage, setRowsPerPage] = useState(1);
    const [totalPages, setTotalPages] = useState(0);
    const [firstPage, setFirstPage] = useState(0);

    useEffect(() => {
        const fetchData = async () => {
            const response = await fetch(
                `http://localhost:3000/api/checkpoint?p=${currentPage}&ps=${rowsPerPage}`
            );
            const result = await response.json();
            const totalCheckpoints = result.result.total_pages;
            const firstPage = result.result.absolute_first_page;


            setData(result.result.items[0]);
            setTotalPages(totalCheckpoints);
            setFirstPage(firstPage);
        };

        fetchData();
    }, [currentPage, rowsPerPage]);

    if (!checkpoint) {
        return <div>No checkpoint data available</div>;
    }
    return (
        <>
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
            <Pagination
                currentPage={currentPage}
                firstPage={firstPage}
                totalPages={totalPages}
                setPage={setCurrentPage}
            />
        </>
    );
};

export default CheckpointDetails;