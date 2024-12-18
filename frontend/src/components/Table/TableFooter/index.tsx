import React, { useEffect, useState } from "react";
import styles from "./TableFooter.module.css";

interface TableFooterProps {
    currentPage: number;    // Current page number
    totalPages: number;     // Total pages available
    setPage: (page: number) => void; // Function to update the page
}

const TableFooter: React.FC<TableFooterProps> = ({ currentPage, totalPages, setPage }) => {
    const pageWindowSize = 3; // Number of pages to show in the window
    const startPage = Math.max(1, currentPage - Math.floor(pageWindowSize / 2));
    const endPage = Math.min(totalPages, startPage + pageWindowSize - 1);

    const [editablePage, setEditablePage] = useState<number | string>(currentPage);

    // Keep the input value in sync with the currentPage
    useEffect(() => {
        setEditablePage(currentPage);
    }, [currentPage]);

    // Validate and navigate to the new page
    const handlePageChange = () => {
        const page = Number(editablePage);
        if (page >= 1 && page <= totalPages) {
            setPage(page);
        } else {
            alert(`Please enter a page number between 1 and ${totalPages}`);
            setEditablePage(currentPage); // Reset on invalid input
        }
    };

    return (
        <div className={styles.footer}>
            {/* First Button */}
            <button
                className={`${styles.pageButton} ${currentPage === 1 ? styles.disabled : ""}`}
                onClick={() => setPage(1)}
                disabled={currentPage === 1}
            >
                «
            </button>

            {/* Previous Button */}
            <button
                className={`${styles.pageButton} ${currentPage === 1 ? styles.disabled : ""}`}
                onClick={() => setPage(currentPage - 1)}
                disabled={currentPage === 1}
            >
                ‹
            </button>

            {/* Page Buttons */}
            {Array.from({ length: endPage - startPage + 1 }, (_, index) => {
                const page = startPage + index;
                return page === currentPage ? (
                    <input
                        key={page}
                        className={styles.pageInput}
                        value={editablePage}
                        onChange={(e) => setEditablePage(e.target.value)}
                        onBlur={handlePageChange}
                        onKeyDown={(e) => e.key === "Enter" && handlePageChange()}
                    />
                ) : (
                    <button
                        key={page}
                        className={styles.pageButton}
                        onClick={() => setPage(page)}
                    >
                        {page}
                    </button>
                );
            })}

            {/* Next Button */}
            <button
                className={`${styles.pageButton} ${currentPage === totalPages ? styles.disabled : ""}`}
                onClick={() => setPage(currentPage + 1)}
                disabled={currentPage === totalPages}
            >
                ›
            </button>

            {/* Last Button */}
            <button
                className={`${styles.pageButton} ${currentPage === totalPages ? styles.disabled : ""}`}
                onClick={() => setPage(totalPages)}
                disabled={currentPage === totalPages}
            >
                »
            </button>
            <div className={styles.pageInfo}>
                Page {currentPage} of {totalPages}
            </div>
        </div>
    );
};

export default TableFooter;