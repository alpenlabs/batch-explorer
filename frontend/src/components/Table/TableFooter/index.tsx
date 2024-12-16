import { useEffect } from "react";

import styles from "./TableFooter.module.css";

// Define the props for TableFooter
interface TableFooterProps {
    range: number[];                  // Array of page numbers
    setPage: (page: number) => void;  // Function to set the current page
    page: number;                     // Current page number
    slice: any[];                     // Array of data for the current page
}

const TableFooter: React.FC<TableFooterProps> = ({ range, setPage, page, slice }) => {
    useEffect(() => {
        if (slice.length < 1 && page !== 1) {
            setPage(page - 1);
        }
    }, [slice, page, setPage]);

    return (
        <div className={styles.tableFooter}>
            {range.map((el, index) => (
                <button
                    key={index}
                    className={`${styles.button} ${page === el ? styles.activeButton : styles.inactiveButton}`}
                    onClick={() => setPage(el)}
                >
                    {el}
                </button>
            ))}
        </div>
    );
};

export default TableFooter;