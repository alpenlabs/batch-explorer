import { useState } from "react";
import styles from "../styles/Header.module.css";

const Header = () => {
    const [isMenuOpen, setIsMenuOpen] = useState(false);

    const toggleMenu = () => {
        setIsMenuOpen((prev) => !prev);
    };
    return (
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
                    <a href="#open-form" className={styles.devnetButton}>Access testnet</a>
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
    )
}
export default Header;