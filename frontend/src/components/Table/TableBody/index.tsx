import { RpcCheckpointInfo } from "../../../types/types";
import styles from "./Table.module.css";
// Define the props for the Table component

interface TableBodyProps {
    data: RpcCheckpointInfo[],
}

const TableBody: React.FC<TableBodyProps> = ({
    data,
    // setRowsPerPage
}) => {
    return (
        <>
            {/* <div className={styles.select_container}>
                <span className={styles.select_info}>Checkpoints per page </span>
                <select className={styles.select} onChange={(e) => {
                    rowsPerPage = setRowsPerPage(parseInt(e.target.value, 10));
                    setPage(1);
                }}>
                    <option>2</option>
                    <option>5</option>
                    <option>10</option>
                </select>
            </div> */}
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
                            {/* <td className={styles.tableCell}>
                                <a
                                    href={`https://blockscoutb86fae58ae.devnet-annapurna.stratabtc.org/block/${checkpoint.l2_blockid}`}
                                    target="_blank"
                                    rel="noreferrer"
                                >
                                    {shortenBlockId(checkpoint.l2_blockid)}
                                </a>
                            </td> */}
                        </tr>
                    ))}
                </tbody >
            </table >

        </>)
};

export default TableBody;