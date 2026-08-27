"use client";

import { useEffect, useState } from "react";
import styles from "./page.module.css";

export default function Home() {
  const [health, setHealth] = useState(null);

  useEffect(() => {
    fetch("/api/health")
      .then((res) => res.json())
      .then(setHealth)
      .catch(() => setHealth({ status: "error" }));
  }, []);

  return (
    <div className={styles.page}>
      <main className={styles.main}>
        <h1>AI Sandbox Next.js Demo</h1>
        <p>
          Unlike the <code className={styles.code}>securenote-web</code> +{" "}
          <code className={styles.code}>securenote-api</code> demo, this app
          runs frontend and backend in a single Next.js process. The route
          handler at <code className={styles.code}>src/app/api/health</code>{" "}
          still reads a secret file server-side — see{" "}
          <code className={styles.code}>secrets/README.md</code> for how it
          stays hidden from the AI Sandbox container.
        </p>
        <div className={styles.status}>
          {health ? JSON.stringify(health, null, 2) : "Loading /api/health..."}
        </div>
      </main>
    </div>
  );
}
