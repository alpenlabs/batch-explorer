import { Route, Routes } from "react-router-dom";
import styles from "../../styles/App.module.css";
import { isRpcCheckpointInfo } from "../../utils/lib";
import CheckpointDetails from "../CheckpointDetails";
import TableBody from "../Table/TableBody";
import Pagination from "./Pagination";
// Define the props for the Table component

interface PaginatedDataProps<T> {
    data: T[],
    // rowsPerPage: number;
    currentPage: number;
    totalPages: number;
    setPage: (page: number) => void; // Function to update the current page
    // setRowsPerPage: (rows: number) => number; // Function to update the rows per page
}

const PaginatedData = <T,>({
    data,
    currentPage,
    totalPages,
    setPage,
}: PaginatedDataProps<T>) => {
    return (

        <div className={styles.wrapper}>
            <Routes>
                <Route
                    path="/"
                    element={
                        <>
                            {/* if type is TableBody */}
                            {isRpcCheckpointInfo(data) ? (
                                <TableBody data={data} />
                            ) : (
                                <div>Unknown data type</div>
                            )}
                        </>
                    }
                />
                <Route
                    path="/checkpoint"
                    element={
                        <>
                            <CheckpointDetails />
                        </>
                    }
                />
            </Routes>

            <Pagination
                currentPage={currentPage}
                totalPages={totalPages}
                setPage={setPage}
            />
        </div >
    );
};

export default PaginatedData;
export type { PaginatedDataProps };
