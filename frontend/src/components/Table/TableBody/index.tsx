import { useEffect, useState } from "react";
import { useSearchParams } from "react-router-dom";
import { RpcCheckpointInfo } from "../../../types";
import Pagination from "../../Paginator/Pagination";
import styles from "./Table.module.css";

const TableBody: React.FC = () => {
    const [data, setData] = useState<RpcCheckpointInfo[]>([]);
    const [rowsPerPage] = useState(10); // Fixed value
    const [totalPages, setTotalPages] = useState(0);
    const [firstPage, setFirstPage] = useState(1);
    const [searchParams, setSearchParams] = useSearchParams();

    // Get `p` from URL and ensure it's a valid number
    const pageFromUrl = Number(searchParams.get("p")) || 1;
    const [currentPage, setCurrentPage] = useState(pageFromUrl);


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
                const response = await fetch(
                    `http://localhost:3000/api/checkpoints?p=${currentPage}&ps=${rowsPerPage}`
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
                            <td className={styles.tableCell}>
                                {checkpoint.batch_txid}
                            </td>
                            <td className={styles.tableCell}>
                                <a href={`/checkpoint?p=${checkpoint.idx}`}>{checkpoint.idx}</a>
                            </td>
                            <td className={styles.tableCell}>{checkpoint.status}</td>
                            <td className={styles.tableCell}>
                                <a href={`https://mempool0713bb23.devnet-annapurna.stratabtc.org/block/${checkpoint.l1_range[0]}`} target="_blank" rel="noreferrer">
                                    {checkpoint.l1_range[0]}
                                </a>
                            </td>
                            <td className={styles.tableCell}>
                                <a href={`https://mempool0713bb23.devnet-annapurna.stratabtc.org/block/${checkpoint.l1_range[1]}`} target="_blank" rel="noreferrer">
                                    {checkpoint.l1_range[1]}
                                </a>
                            </td>
                            <td className={styles.tableCell}>
                                <a href={`https://blockscoutb86fae58ae.devnet-annapurna.stratabtc.org/block/${checkpoint.l2_range[0]}`} target="_blank" rel="noreferrer">
                                    {checkpoint.l2_range[0]}
                                </a>
                            </td>
                            <td className={styles.tableCell}>
                                <a href={`https://blockscoutb86fae58ae.devnet-annapurna.stratabtc.org/block/${checkpoint.l2_range[1]}`} target="_blank" rel="noreferrer">
                                    {checkpoint.l2_range[1]}
                                </a>
                            </td>
                        </tr>
                    ))}
                </tbody>
            </table>
            <Pagination
                currentPage={currentPage}
                firstPage={firstPage}
                totalPages={totalPages}
                setPage={setPage}
            />
        </>
    );
};

export default TableBody;