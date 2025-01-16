import { useEffect, useState } from "react";
import { BrowserRouter as Router } from "react-router-dom";
import Header from "./components/Header";
import PaginatedData from "./components/Paginator";
import SearchSection from "./components/SearchSection";
import styles from "./styles/App.module.css";
const App = () => {
  const [data, setData] = useState([]);
  const [currentPage, setCurrentPage] = useState(1);
  const [rowsPerPage, setRowsPerPage] = useState(10);
  const [totalPages, setTotalPages] = useState(0);
  const [isMenuOpen, setIsMenuOpen] = useState(false);

  const toggleMenu = () => {
    setIsMenuOpen((prev) => !prev);
  };

  useEffect(() => {
    const fetchData = async () => {
      const response = await fetch(
        `http://localhost:3000/api/checkpoints?p=${currentPage}&ps=${rowsPerPage}`
      );
      const result = await response.json();
      const totalCheckpoints = result.result.total_pages;

      setData(result.result.items);
      setTotalPages(totalCheckpoints);
    };

    fetchData();
  }, [currentPage, rowsPerPage]);

  return (
    <Router>
      <main className={styles.container}>
        <Header />
        <SearchSection />
        <PaginatedData
          data={data}
          // rowsPerPage={rowsPerPage}
          currentPage={currentPage}
          totalPages={totalPages}
          setPage={setCurrentPage}
        />
      </main >
    </Router>
  );
};

export default App;