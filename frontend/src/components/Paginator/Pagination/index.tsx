import React, { useEffect, useState } from "react";
import { useNavigate, useSearchParams } from "react-router-dom";
// import { useAlert } from "../../../hooks/useAlert";
import styles from "./Pagination.module.css";

interface PaginationProps {
    currentPage: number;
    firstPage: number;
    totalPages: number;
    setPage: (page: number) => void;
}

const Pagination: React.FC<PaginationProps> = ({ currentPage, firstPage, totalPages, setPage }) => {
    // const { showAlert, alertUI } = useAlert();
    const pageWindowSize = 1;
    const startPage = Math.max(firstPage, currentPage - Math.floor(pageWindowSize / 2));
    const endPage = Math.min(totalPages, startPage + pageWindowSize - 1);

    const navigate = useNavigate();
    const [searchParams, setSearchParams] = useSearchParams();
    const [editablePage, setEditablePage] = useState<number | string>(currentPage);

    // Sync the input value with currentPage
    useEffect(() => {
        setEditablePage(currentPage);
    }, [currentPage]);

    // Function to update both state and URL
    const updatePage = (page: number) => {
        if (page < firstPage || page > totalPages) return;
        setPage(page);
        setSearchParams({ p: page.toString() }); // Update the URL query
        navigate(`?p=${page}`, { replace: true }); // Prevent adding to browser history stack
    };

    // Validate and navigate to new page on input change
    const handlePageChange = () => {
        const page = Number(editablePage);
        if (page >= firstPage && page <= totalPages) {
            updatePage(page);
        } else {
            // showAlert(`Please enter a page number between ${firstPage} and ${totalPages}`);
            return;
            // setEditablePage(currentPage); // Reset on invalid input
        }
    };

    return (
        <>

            <div className={styles.footer}>
                {/* First Button */}
                <button
                    className={`${styles.pageButton} ${currentPage === firstPage ? styles.disabled : ""}`}
                    onClick={() => updatePage(firstPage)}
                    disabled={currentPage === firstPage}
                >
                    «
                </button>

                {/* Previous Button */}
                <button
                    className={`${styles.pageButton} ${currentPage === firstPage ? styles.disabled : ""}`}
                    onClick={() => updatePage(currentPage - 1)}
                    disabled={currentPage === firstPage}
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
                            onClick={() => updatePage(page)}
                        >
                            {page}
                        </button>
                    );
                })}

                {/* Next Button */}
                <button
                    className={`${styles.pageButton} ${currentPage === totalPages ? styles.disabled : ""}`}
                    onClick={() => updatePage(currentPage + 1)}
                    disabled={currentPage === totalPages}
                >
                    ›
                </button>

                {/* Last Button */}
                <button
                    className={`${styles.pageButton} ${currentPage === totalPages ? styles.disabled : ""}`}
                    onClick={() => updatePage(totalPages)}
                    disabled={currentPage === totalPages}
                >
                    »
                </button>

                <div className={styles.pageInfo}>
                    Page {currentPage} of {totalPages}
                </div>
            </div>
        </>
    );
};

export default Pagination;