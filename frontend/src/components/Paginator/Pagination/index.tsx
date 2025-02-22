import React, { useEffect, useState } from "react";
import { useNavigate, useSearchParams } from "react-router-dom";
// import { useAlert } from "../../../hooks/useAlert";
import styles from "../../../styles/Pagination.module.css";
import AlertComponent from "../../Alert";
interface PaginationProps {
    currentPage: number;
    firstPage: number;
    totalPages: number;
    setPage: (page: number) => void;
}

const Pagination: React.FC<PaginationProps> = ({ currentPage, firstPage, totalPages, setPage }) => {
    const pageWindowSize = 1;
    const startPage = Math.max(firstPage, currentPage - Math.floor(pageWindowSize / 2));
    const endPage = Math.min(totalPages, startPage + pageWindowSize - 1);

    const navigate = useNavigate();
    const [searchParams, setSearchParams] = useSearchParams();
    const [editablePage, setEditablePage] = useState<number | string>(currentPage);
    const [showAlert, setShowAlert] = useState(false);

    // Sync the input value with currentPage
    useEffect(() => {
        setEditablePage(currentPage);
    }, [currentPage]);

    // Function to update both state and URL
    const updatePage = (page: number) => {
        if (page >= firstPage && page <= totalPages) {
            setPage(page);
            setSearchParams({ p: page.toString() }); // Update the URL query
            navigate(`?p=${page}`, { replace: true }); // Prevent adding to browser history stack
        } else {
            setShowAlert(true);
            setTimeout(() => {
                // wait for 3 seconds and then set the alert to false
                setShowAlert(false);
            }, 2000);
            return;
        }
    };

    // Validate and navigate to new page on input change
    const handlePageChange = () => {
        const page = Number(editablePage);
        updatePage(page);
    };

    return (
        <>

            <div className={styles.footer}>
                <div className={styles.pageButtons}>
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
                </div>
                {
                    totalPages > 0 &&
                    <div className={styles.pageInfo}>
                        Page {currentPage} of {totalPages}
                    </div>
                }
                {
                    totalPages === 0 &&
                    <div className={styles.pageInfo}>
                        No data found
                    </div>}
            </div>
            <div className={styles.alertWrapper}>
                {showAlert && <AlertComponent />}
            </div>
        </>
    );
};
export default Pagination;