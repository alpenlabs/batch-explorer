import { PaginatedDataProps } from "../Paginator";
import TableBody from "./TableBody";
// Define the props for the Table component

const Table: React.FC<PaginatedDataProps> = ({
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
        </>
    );
};

export default Table;
