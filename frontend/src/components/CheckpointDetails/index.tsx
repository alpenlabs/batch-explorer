import { useEffect, useState } from "react";
import { useSearchParams } from "react-router-dom";
import styles from "../../styles/CheckpointDetails.module.css";
import { RpcCheckpointInfo } from "../../types";
import Pagination from "../Paginator/Pagination/index";

const CheckpointDetails = () => {
    const [searchParams] = useSearchParams();
    const page = searchParams.get("p"); // Get the "p" query parameter

    // Ensure `currentPage` updates when `p` changes
    const [currentPage, setCurrentPage] = useState<number>(Number(page) || 0);
    const [checkpoint, setData] = useState<RpcCheckpointInfo | null>(null);
    const [totalPages, setTotalPages] = useState(0);
    const [firstPage, setFirstPage] = useState(0);
    const rowsPerPage = 1; // Fixed value

    useEffect(() => {
        // Convert the query param `p` to a number
        const pageNumber = Number(page);
        if (!isNaN(pageNumber) && pageNumber !== currentPage) {
            setCurrentPage(pageNumber);
        }
    }, [page]);

    useEffect(() => {
        console.log("currentPage", currentPage);
        const fetchData = async () => {
            try {
                const response = await fetch(
                    `http://localhost:3000/api/checkpoint?p=${currentPage}`
                );
                const result = await response.json();
                setData(result.result.items[0]);
                console.log("result", result);
                setTotalPages(result.result.total_pages);
                setFirstPage(result.result.absolute_first_page);
            } catch (error) {
                console.error("Error fetching checkpoint data:", error);
            }
        };
        if (currentPage >= 0) fetchData();
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