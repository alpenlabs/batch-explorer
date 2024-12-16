// @/src/App.jsx
import { useState } from "react";

import styles from "./App.module.css";
import Table from "./components/Table";
import batchesData from "./data/batches.ts";

const App = () => {
  const [batches] = useState([...batchesData]);
  return (
    <main className={styles.container}>
      <div className={styles.wrapper}>
        <Table data={batches} rowsPerPage={4} />
      </div>
    </main>
  );
};

export default App;
