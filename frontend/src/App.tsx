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

      setData(result.checkpoints);
      setTotalPages(Math.ceil(totalCheckpoints / rowsPerPage));
    };

    fetchData();
  }, [currentPage, rowsPerPage]);

  return (
    <main className={styles.container}>
      <header className={styles.header}>
        {/* Logo Wrapper */}
        <a href="/" className={styles.logoWrapper}>
          <div className={styles.logoSvg}>
            <img src="/Strata_full_logo_sand.png" alt="STRATA" />
          </div>
        </a>

        {/* Navbar Menu */}
        <div className={styles.navbarMenuWrapper}>
          <nav className={styles.navMenu} role="navigation">
            <div className={styles.navLinks}>
              <a href="#why" className={styles.navLink}>Why Strata</a>
              <a href="https://docs.stratabtc.org/" target="_blank" className={styles.navLink}>Documentation</a>
              <a href="#blog" className={styles.navLink}>Blog</a>
            </div>
          </nav>
          <div className={styles.devnetButtonWrapper}>
            <a href="#open-form" className={styles.devnetButton}>Access DevNet</a>
          </div>
        </div>

        {/* Menu Button for Mobile */}
        <div className={styles.menuButton} role="button" aria-label="menu" aria-haspopup="menu" aria-expanded="false">
          <div className={styles.burger}>
            <div className={styles.burgerBar}></div>
            <div className={styles.burgerBar}></div>
            <div className={styles.burgerBar}></div>
          </div>
        </div>
      </header>
      <div className={styles.searchSection}>
        <h1 className={styles.title}>Batch Explorer</h1>
        <div className={styles.searchBox}>
          <input
            type="text"
            placeholder="🔍 Search by Strata block number or block hash"
            className={styles.searchInput}
          />
        </div>
      </div>
      <div className={styles.wrapper}>
        <Table
          data={data}
          rowsPerPage={rowsPerPage}
          currentPage={currentPage}
          totalPages={totalPages}
          setPage={setCurrentPage}
          setRowsPerPage={(rows) => {
            setRowsPerPage(rows);
            return rows;
          }}
        />
      </div>
    </main>
  );
};

export default App;