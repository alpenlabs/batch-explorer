import { useEffect, useState } from "react";
import PaginatedData from "./components/Paginator";
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
    <main className={styles.container}>
      <header className={styles.header}>
        {/* Logo Wrapper */}
        <a href="/" className={styles.logoWrapper}>
          <div className={styles.logoSvg}>
            <img src="/Strata_full_logo_sand.png" alt="STRATA" />
          </div>
        </a>

        {/* Navbar Menu */}
        <div className={`${styles.navbarMenuWrapper} ${isMenuOpen ? styles.showMenu : ""
          }`}>
          <nav className={styles.navMenu} role="navigation">
            <div className={styles.navLinks}>
              <a href="https://docs.stratabtc.org/" target="_blank" className={styles.navLink}>Documentation</a>
              <a href="#blog" className={styles.navLink}>Blog</a>
            </div>
          </nav>
          <div className={styles.devnetButtonWrapper}>
            <a href="#open-form" className={styles.devnetButton}>Access DevNet</a>
          </div>
        </div>

        {/* Menu Button for Mobile */}
        <div
          className={styles.menuButton}
          role="button"
          aria-label={isMenuOpen ? "close menu" : "open menu"}
          aria-haspopup="menu"
          onClick={toggleMenu}
        >
          {isMenuOpen ? (
            <div className={styles.cross}>
              <div className={styles.crossBar}></div>
              <div className={styles.crossBar}></div>
            </div>
          ) : (
            <div className={styles.burger}>
              <div className={styles.burgerBar}></div>
              <div className={styles.burgerBar}></div>
              <div className={styles.burgerBar}></div>
            </div>
          )}
        </div>
      </header>
      <div className={styles.searchSection}>
        <a href="/"><h1 className={styles.title}>Batch explorer</h1></a>
        <div className={styles.searchBox}>
          <input
            type="text"
            placeholder="🔍 Search by Strata orchestration layer block number or block hash"
            className={styles.searchInput}
          />
        </div>
      </div>
      <div className={styles.wrapper}>
        <PaginatedData
          data={data}
          // rowsPerPage={rowsPerPage}
          currentPage={currentPage}
          totalPages={totalPages}
          setPage={setCurrentPage}
        />
      </div>
    </main>
  );
};

export default App;