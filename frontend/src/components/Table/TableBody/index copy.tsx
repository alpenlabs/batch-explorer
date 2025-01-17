import { useEffect, useState } from "react";
import { useSearchParams } from "react-router-dom";
import { RpcCheckpointInfo } from "../../../types";
import Pagination from "../../Paginator/Pagination";
import styles from "./Table.module.css";
// Define the props for the Table component


const TableBody: React.FC = ({ }) => {
    const [data, setData] = useState<RpcCheckpointInfo[]>([]);
    const [currentPage, setCurrentPage] = useState(1);
    const [rowsPerPage, setRowsPerPage] = useState(10);
    const [totalPages, setTotalPages] = useState(0);
    const [firstPage, setFirstPage] = useState(1);
    const [searchParams, setSearchParams] = useSearchParams();

    useEffect(() => {
        const fetchData = async () => {
            const response = await fetch(
                `http://localhost:3000/api/checkpoints?p=${currentPage}&ps=${rowsPerPage}`
            );
            const result = await response.json();
            const totalCheckpoints = result.result.total_pages;
            const firstPage = result.result.absolute_first_page;

            setData(result.result.items);
            setTotalPages(totalCheckpoints);
            setFirstPage(firstPage);
        };

        fetchData();
    }, [currentPage, rowsPerPage]);

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
                            <td className={styles.tableCell}>
                                {checkpoint.status}
                            </td>
                            {/* TODO: update urls from env or config */}
                            < td className={styles.tableCell} >
                                <a href={`https://mempool0713bb23.devnet-annapurna.stratabtc.org/block/${checkpoint.l1_range[0]}`} target="_blank" rel="noreferrer">
                                    {checkpoint.l1_range[0]}
                                </a>
                            </td>
                            <td className={styles.tableCell}>
                                <a href={`https://mempool0713bb23.devnet-annapurna.stratabtc.org/block/${checkpoint.l1_range[0]}`} target="_blank" rel="noreferrer">
                                    {checkpoint.l1_range[1]}
                                </a>
                            </td>
                            <td className={styles.tableCell}>
                                <a href={`https://blockscoutb86fae58ae.devnet-annapurna.stratabtc.org/block/${checkpoint.l2_range[0]}`} target="_blank" rel="noreferrer">
                                    {checkpoint.l2_range[0]}
                                </a>
                            </td>
                            <td className={styles.tableCell}>
                                <a href={`https://blockscoutb86fae58ae.devnet-annapurna.stratabtc.org/block/${checkpoint.l2_range[0]}`} target="_blank" rel="noreferrer">
                                    {checkpoint.l2_range[1]}
                                </a>
                            </td>
                        </tr>
                    ))}
                </tbody >
            </table >
            <Pagination
                currentPage={currentPage}
                firstPage={firstPage}
                totalPages={totalPages}
                setPage={setCurrentPage}
            />
        </>)
};

export default TableBody;
