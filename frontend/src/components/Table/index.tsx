import shortenBlockId from "../../utils/lib";
import styles from "./Table.module.css";
import TableFooter from "./TableFooter";
// Define the props for the Table component
interface RpcCheckpointInfo {
    idx: number;               // Index of the checkpoint
    l1_range: [number, number]; // L1 height range (start, end)
    l2_range: [number, number]; // L2 height range (start, end)
    l2_blockid: string;        // L2 block ID
}

interface TableProps {
    data: RpcCheckpointInfo[]; // Array of checkpoint objects
    rowsPerPage: number;       // Rows per page
    currentPage: number;       // Current page number
    totalPages: number;        // Total pages available
    setPage: (page: number) => void; // Function to update the current page
    setRowsPerPage: (rows: number) => number; // Function to update the rows per page
}

const Table: React.FC<TableProps> = ({ data, rowsPerPage, currentPage, totalPages, setPage, setRowsPerPage }) => {
    return (
        <>
            <div className={styles.select_container}>
                <span className={styles.select_info}>Checkpoints per page </span>
                <select className={styles.select} onChange={(e) => {
                    rowsPerPage = setRowsPerPage(parseInt(e.target.value, 10));
                    setPage(1);
                }}>
                    <option>2</option>
                    <option>5</option>
                    <option>10</option>
                </select>
            </div>
            <table className={styles.table}>
                <thead className={styles.tableRowHeader}>
                    <tr>
                        <th className={styles.tableHeader}>Index</th>
                        <th className={styles.tableHeader}>L1 Range</th>
                        <th className={styles.tableHeader}>L2 Range</th>
                        <th className={styles.tableHeader}>L2 Block ID</th>
                    </tr>
                </thead>
                <tbody>
                    {data.map((checkpoint) => (
                        <tr className={styles.tableRowItems} key={checkpoint.idx}>
                            <td className={styles.tableCell}>{checkpoint.idx}</td>
                            {/* TODO: update urls from env or config */}
                            <td className={styles.tableCell}>
                                <a href={`https://mempool0713bb23.devnet-annapurna.stratabtc.org/block/${checkpoint.l1_range[0]}`} target="_blank" rel="noreferrer">
                                    {checkpoint.l1_range[0]}
                                </a>
                                -
                                <a href={`https://mempool0713bb23.devnet-annapurna.stratabtc.org/block/${checkpoint.l1_range[0]}`} target="_blank" rel="noreferrer">
                                    {checkpoint.l1_range[1]}
                                </a>
                            </td>
                            <td className={styles.tableCell}>
                                <a href={`https://blockscoutb86fae58ae.devnet-annapurna.stratabtc.org/block/${checkpoint.l2_range[0]}`} target="_blank" rel="noreferrer">
                                    {checkpoint.l2_range[0]}
                                </a>
                                -
                                <a href={`https://blockscoutb86fae58ae.devnet-annapurna.stratabtc.org/block/${checkpoint.l2_range[0]}`} target="_blank" rel="noreferrer">
                                    {checkpoint.l2_range[1]}
                                </a>
                            </td>
                            <td className={styles.tableCell}>
                                <a
                                    href={`https://blockscoutb86fae58ae.devnet-annapurna.stratabtc.org/block/${checkpoint.l2_blockid}`}
                                    target="_blank"
                                    rel="noreferrer"
                                >
                                    {shortenBlockId(checkpoint.l2_blockid)}
                                </a>
                            </td>
                        </tr>
                    ))}
                </tbody>
            </table >
            <TableFooter
                currentPage={currentPage}
                totalPages={totalPages}
                setPage={setPage}
            />
        </>
    );
};

export default Table;