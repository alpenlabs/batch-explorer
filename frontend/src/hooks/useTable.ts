// @/src/hooks/useTable.js

import { useEffect, useState } from "react";

const calculateRange = (data: any[], rowsPerPage: number): number[] => {
    const range: number[] = [];
    const num = Math.ceil(data.length / rowsPerPage);
    for (let i = 1; i <= num; i++) {
        range.push(i);
    }
    return range;
};

const sliceData = <T>(data: T[], page: number, rowsPerPage: number): T[] => {
    return data.slice((page - 1) * rowsPerPage, page * rowsPerPage);
};

const useTable = <T>(data: T[], page: number, rowsPerPage: number) => {
    const [tableRange, setTableRange] = useState<number[]>([]);
    const [slice, setSlice] = useState<T[]>([]);

    useEffect(() => {
        const range = calculateRange(data, rowsPerPage);
        setTableRange([...range]);

        const slice = sliceData(data, page, rowsPerPage);
        setSlice([...slice]);
    }, [data, rowsPerPage, page]);

    return { slice, range: tableRange };
};

export default useTable;





