
import React, { createContext, useEffect, useState } from "react";

export interface AppConfig {
    apiBaseUrl: string;
    alpenExplorerBaseUrl: string;
    bitcoinExplorerBaseUrl: string;
    refreshIntervalS: number,
    environment: string;
}

const ConfigContext = createContext<AppConfig | null>(null);

export const ConfigProvider: React.FC<{ children: React.ReactNode }> = ({
    children,
}) => {
    const [config, setConfig] = useState<AppConfig | null>(null);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        fetch("/config.json")
            .then((res) => {
                if (!res.ok) throw new Error("Failed to load config");
                return res.json();
            })
            .then(setConfig)
            .catch((err) => {
                console.error("Config load error:", err);
                setError("Failed to load configuration");
            });
    }, []);

    if (error) return <div>Error: {error}</div>;
    if (!config) return <div>Loading config...</div>;

    return (
        <ConfigContext.Provider value={config}>
            {children}
        </ConfigContext.Provider>
    );
};

export default ConfigContext;
