import { RpcCheckpointInfo } from "../../types/types";
import TableBody from "../Table/TableBody";
import Pagination from "./Pagination";
// Define the props for the Table component

interface PaginatedDataProps {
    data: RpcCheckpointInfo[],
    // rowsPerPage: number;
    currentPage: number;
    totalPages: number;
    setPage: (page: number) => void; // Function to update the current page
    // setRowsPerPage: (rows: number) => number; // Function to update the rows per page
}

const PaginatedData: React.FC<PaginatedDataProps> = ({
    data,
    // rowsPerPage,
    currentPage,
    totalPages,
    setPage,
    // setRowsPerPage
}) => {
    return (
        <>
            <TableBody
                data={data}
            />
            <Pagination
                currentPage={currentPage}
                totalPages={totalPages}
                setPage={setPage}
            />
        </>
    );
};

export default PaginatedData;
export type { PaginatedDataProps };
