import { useEffect, useState } from "react";
import { useSearchParams } from "react-router-dom";
import styles from "../../styles/CheckpointDetails.module.css";
import { RpcCheckpointInfoBatchExp } from "../../types";
import { reverseEndian , truncateTxid} from "../../utils/lib";
import Pagination from "../Paginator/Pagination/index";
const CheckpointDetails = () => {
    const [searchParams] = useSearchParams();
    const page = searchParams.get("p"); // Get the "p" query parameter

    // Ensure `currentPage` updates when `p` changes
    const [currentPage, setCurrentPage] = useState<number>(Number(page) || 0);
    const [checkpoint, setData] = useState<RpcCheckpointInfoBatchExp | null>(null);
    const [totalPages, setTotalPages] = useState(0);
    const [firstPage, setFirstPage] = useState(0);
    const rowsPerPage = 1; // Fixed value

    const MEMPOOL_BASE_URL = import.meta.env.VITE_MEMPOOL_BASE_URL || "https://default-mempool-url.com";
    const BLOCKSCOUT_BASE_URL = import.meta.env.VITE_BLOCKSCOUT_BASE_URL || "https://default-blockscout-url.com";

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
                const baseUrl = import.meta.env.VITE_API_BASE_URL || 'http://localhost:3000';
                const response = await fetch(
                    `${baseUrl}/api/checkpoint?p=${currentPage}`
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
        return <div className={styles.noData}>Loading...</div>
    }
    return (
        <>
            <div className={styles.checkpointContainer}>
                <div className={styles.checkpointRow}>
                    <span className={styles.checkpointLabel}>Batch TXID:</span>
                    <a
                        href={`${MEMPOOL_BASE_URL}tx/${reverseEndian(checkpoint.l1_reference?.txid)}`}
                        target="_blank"
                        rel="noreferrer"
                    >
                      {truncateTxid(reverseEndian(checkpoint.l1_reference?.txid))}
                    </a>

                </div>
                <div className={styles.checkpointRow}>
                    <span className={styles.checkpointLabel}>Epoch index:</span>
                    <span className={styles.checkpointValue}>{checkpoint.idx}</span>
                </div>
                <div className={styles.checkpointRow}>
                    <span className={styles.checkpointLabel}>Status:</span>
                    <span className={styles.checkpointValue}>{checkpoint.confirmation_status}</span>
                </div>
                <div className={styles.checkpointRow}>
                    <span className={styles.checkpointLabel}>Signet start block:</span>
                    <span className={styles.checkpointValue}>
                        <a
                            href={`${MEMPOOL_BASE_URL}block/${checkpoint.l1_range[0]}`}
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
                            href={`${MEMPOOL_BASE_URL}block/${checkpoint.l1_range[1]}`}
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
                            href={`${BLOCKSCOUT_BASE_URL}block/${checkpoint.l2_range[0]}`}
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
                            href={`${BLOCKSCOUT_BASE_URL}block/${checkpoint.l2_range[1]}`}
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
