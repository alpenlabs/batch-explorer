import { useState } from "react";

import { Batch } from "../../data/batches";
import useTable from "../../hooks/useTable";
import styles from "./Table.module.css";
import TableFooter from "./TableFooter";

// Define the props for the Table component
interface TableProps {
    data: Batch[];         // An array of Batch objects
    rowsPerPage: number;   // A number specifying rows per page
}

const Table: React.FC<TableProps> = ({ data, rowsPerPage }) => {
    const [page, setPage] = useState<number>(1);
    const { slice, range } = useTable(data, page, rowsPerPage);

    return (
        <>
            <table className={styles.table}>
                <thead className={styles.tableRowHeader}>
                    <tr>
                        <th className={styles.tableHeader}>L1 Start Hash</th>
                        <th className={styles.tableHeader}>L1 End Hash</th>
                        <th className={styles.tableHeader}>L2 Start Height</th>
                        <th className={styles.tableHeader}>L2 End Height</th>
                        <th className={styles.tableHeader}>Status</th>
                        <th className={styles.tableHeader}>Commitment</th>
                        <th className={styles.tableHeader}>Commitment Tx ID</th>
                        <th className={styles.tableHeader}>Proof Tx ID</th>
                    </tr>
                </thead>
                <tbody>
                    {slice.map((el) => (
                        <tr className={styles.tableRowItems} key={el.l1StartHash}>
                            <td className={styles.tableCell}>{el.l1StartHash}</td>
                            <td className={styles.tableCell}>{el.l1EndHash}</td>
                            <td className={styles.tableCell}>{el.l2StartHeight}</td>
                            <td className={styles.tableCell}>{el.l2EndHeight}</td>
                            <td className={styles.tableCell}>{el.status}</td>
                            <td className={styles.tableCell}>{el.commitment}</td>
                            <td className={styles.tableCell}>
                                <a href={`https://explorer.com/tx/${el.commitmentTransactionID}`} target="_blank" rel="noopener noreferrer">
                                    {el.commitmentTransactionID}
                                </a>
                            </td>
                            <td className={styles.tableCell}>{el.proofTransactionID || "N/A"}</td>
                        </tr>
                    ))}
                </tbody>
            </table>
            <TableFooter range={range} slice={slice} setPage={setPage} page={page} />
        </>
    );
};

export default Table;