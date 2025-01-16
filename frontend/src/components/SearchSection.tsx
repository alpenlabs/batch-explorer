import styles from '../styles/SearchSection.module.css';

const SearchSection = () => (
    <div className={styles.searchSection}>
        <a href="/"><h1 className={styles.title}>Batch explorer</h1></a>
        <form action="/search" method="get" className={styles.searchBox}>
            <input
                type="text"
                name="query"
                placeholder="🔍 Search by Strata orchestration layer block number or block hash"
                className={styles.searchInput}
                required
            />
        </form>
    </div>

)

export default SearchSection;