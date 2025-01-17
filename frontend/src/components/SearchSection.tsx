import { useState } from "react";
import { useNavigate } from "react-router-dom";
import styles from '../styles/SearchSection.module.css';

const SearchSection = () => {
    const [query, setQuery] = useState("");
    const navigate = useNavigate();

    const handleSearch = async (e: React.FormEvent) => {
        e.preventDefault(); // Prevent full page reload
        if (!query.trim()) return; // Prevent empty searches

        try {
            const response = await fetch(
                `http://localhost:3000/api/search?query=${query.trim()}&ps=1`
            );
            const result = await response.json();

            if (result.error) {
                console.error("Error fetching data:", result.error);
                return;
            }

            const checkpoint_id = result.result;
            if (checkpoint_id) {
                navigate(`/checkpoint?p=${checkpoint_id}`); // Navigate only after receiving data
            }
        } catch (error) {
            console.error("UNKNOWN Error fetching data:", error);
        }
    };

    return (
        <div className={styles.searchSection}>
            <a href="/"><h1 className={styles.title}>Batch Explorer</h1></a>
            <form onSubmit={handleSearch} className={styles.searchBox}>
                <input
                    type="text"
                    value={query}
                    onChange={(e) => setQuery(e.target.value)}
                    placeholder="🔍 Search by Strata orchestration layer block number or block hash"
                    className={styles.searchInput}
                    required
                />
            </form>
        </div>
    );
};

export default SearchSection;