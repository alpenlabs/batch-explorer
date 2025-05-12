
import { BrowserRouter as Router } from "react-router-dom";
import Header from "./components/Header";
import PaginatedData from "./components/Paginator";
import SearchSection from "./components/SearchSection";
import styles from "./styles/App.module.css";
import { ConfigProvider } from "./providers/ConfigProvider";

const App = () => {
  return (
    <ConfigProvider>
      <Router>
        <main className={styles.container}>
          <Header />
          <SearchSection />
          <PaginatedData />
        </main >
      </Router>
    </ConfigProvider>
  );
};

export default App;