import { RpcCheckpointInfoCheckpointExp } from "../../types";
import TableBody from "./TableBody";
// Define the props for the Table component

interface TableProps {
  data: RpcCheckpointInfoCheckpointExp[];
}

const Table: React.FC<TableProps> = ({}) => {
  return (
    <>
      <TableBody />
    </>
  );
};

export default Table;
