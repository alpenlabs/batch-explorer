import { useEffect, useState } from "react";
import { Link, useSearchParams } from "react-router-dom";
import { RpcCheckpointInfoBatchExp } from "../../../types";
import { truncateTxid, shortenIds } from "../../../utils/lib";
import Pagination from "../../Paginator/Pagination";
import styles from "../../../styles/Table.module.css";
const TableBody: React.FC = () => {
    const [data, setData] = useState<RpcCheckpointInfoBatchExp[]>([]);
    const [rowsPerPage] = useState(10); // Fixed value
    const [totalPages, setTotalPages] = useState(0);
    const [firstPage, setFirstPage] = useState(1);
    const [searchParams, setSearchParams] = useSearchParams();

    // Get `p` from URL and ensure it's a valid number
    const pageFromUrl = Number(searchParams.get("p")) || 1;
    const [currentPage, setCurrentPage] = useState(pageFromUrl);

    const MEMPOOL_BASE_URL = import.meta.env.VITE_MEMPOOL_BASE_URL || "https://default-mempool-url.com";
    const BLOCKSCOUT_BASE_URL = import.meta.env.VITE_BLOCKSCOUT_BASE_URL || "https://default-blockscout-url.com";

    useEffect(() => {
        if (currentPage !== pageFromUrl) {
            setCurrentPage(pageFromUrl);
        }
    }, [pageFromUrl]);

    /** 
     * - Ensures data reloads when the user changes pages.
     */
    useEffect(() => {
        const fetchData = async () => {
            try {
                console.log("fetching data...");
                const baseUrl = import.meta.env.VITE_API_BASE_URL || 'http://localhost:3000';
                const response = await fetch(
                    `${baseUrl}/api/checkpoints?p=${currentPage}&ps=${rowsPerPage}`
                );
                const result = await response.json();
                setData(result.result.items);
                setTotalPages(result.result.total_pages);
                setFirstPage(result.result.absolute_first_page);
            } catch (error) {
                console.error("Error fetching data:", error);
            }
        };

        fetchData();

        const refreshDuration = import.meta.env.VITE_REFRESH_INTERVAL || 30000;
        const interval = setInterval(fetchData, refreshDuration);

        return () => clearInterval(interval); // Clear interval when unmounting or dependencies change
    }, [currentPage, rowsPerPage]); // Trigger fetch when `currentPage` changes

    /** 
     * Immediately update `searchParams` and state 
     * - Prevents the need for a second click.
     */
    const setPage = (page: number) => {
        if (page < firstPage || page > totalPages || page === currentPage) return;

        setSearchParams({ p: page.toString() }); // Update URL first
        setCurrentPage(page); // Then update state immediately
    };

    return (
        <>
            <div className={styles.tableContainer}>
                <table className={styles.table}>
                    <thead className={styles.tableRowHeader}>
                        <tr>
                            <th className={styles.tableHeader}>Batch TXID</th>
                            <th className={styles.tableHeader}>Epoch index</th>
                            <th className={styles.tableHeader}>Status</th>
                            <th className={styles.tableHeader}>Signet start block</th>
                            <th className={styles.tableHeader}>Signet end block</th>
                            <th className={styles.tableHeader}>Strata start block</th>
                            <th className={styles.tableHeader}>Strata end block</th>
                        </tr>
                    </thead>
                    <tbody>
                        {data.map((checkpoint) => (
                            <tr className={styles.tableRowItems} key={checkpoint.idx}>
                                <td className={styles.tableCell} title={checkpoint.l1_reference?.txid}>
                                    <a href={`${MEMPOOL_BASE_URL}tx/${checkpoint.l1_reference?.txid}`} target="_blank" rel="noreferrer">
                                        {shortenIds(truncateTxid(checkpoint.l1_reference?.txid))}
                                    </a>
                                </td>
                                <td className={styles.tableCell}>
                                    <Link to={`/checkpoint?p=${checkpoint.idx}`}>{checkpoint.idx}</Link>
                                </td>
                                <td className={styles.tableCell}>{checkpoint.confirmation_status}</td>
                                <td className={styles.tableCell}>
                                    <a href={`${MEMPOOL_BASE_URL}block/${checkpoint.l1_range[0]}`} target="_blank" rel="noreferrer">
                                        {checkpoint.l1_range[0]}
                                    </a>
                                </td>
                                <td className={styles.tableCell}>
                                    <a href={`${MEMPOOL_BASE_URL}block/${checkpoint.l1_range[1]}`} target="_blank" rel="noreferrer">
                                        {checkpoint.l1_range[1]}
                                    </a>
                                </td>
                                <td className={styles.tableCell}>
                                    <a href={`${BLOCKSCOUT_BASE_URL}block/${checkpoint.l2_range[0]}`} target="_blank" rel="noreferrer">
                                        {checkpoint.l2_range[0]}
                                    </a>
                                </td>
                                <td className={styles.tableCell}>
                                    <a href={`${BLOCKSCOUT_BASE_URL}block/${checkpoint.l2_range[1]}`} target="_blank" rel="noreferrer">
                                        {checkpoint.l2_range[1]}
                                    </a>
                                </td>
                            </tr>
                        ))}
                    </tbody>
                </table>
            </div>
            {
                totalPages == 0 && <div className={styles.noData}>Loading...</div>
            }
            {
                totalPages > 0 && <Pagination
                    currentPage={currentPage}
                    firstPage={firstPage}
                    totalPages={totalPages}
                    setPage={setPage}
                />
            }
        </>
    );
};

export default TableBody;
