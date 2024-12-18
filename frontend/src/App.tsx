import { useEffect, useState } from "react";
import styles from "./App.module.css";
import Table from "./components/Table";

const App = () => {
  const [data, setData] = useState([]);
  const [currentPage, setCurrentPage] = useState(1);
  const [rowsPerPage, setRowsPerPage] = useState(10);
  const [totalPages, setTotalPages] = useState(0);

  useEffect(() => {
    const fetchData = async () => {
      const response = await fetch(
        `http://localhost:3000/checkpoints_paginated?page=${currentPage}&page_size=${rowsPerPage}`
      );
      const result = await response.json();
      const totalCheckpoints = result.total_checkpoints;

      // Update state with the backend response
      setData(result.checkpoints);
      setTotalPages(Math.ceil(totalCheckpoints / rowsPerPage));
    };

    fetchData();
  }, [currentPage, rowsPerPage]);

  return (
    <main className={styles.container}>
      <div className={styles.wrapper}>
        <Table
          data={data}
          rowsPerPage={rowsPerPage}
          currentPage={currentPage}
          totalPages={totalPages}
          setPage={setCurrentPage}
          setRowsPerPage={(rows) => { setRowsPerPage(rows); return rows; }}
        />
      </div>
    </main>
  );
};

export default App;