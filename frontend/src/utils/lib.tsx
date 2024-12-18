const shortenBlockId = (value: string, startLength: number = 8, endLength: number = 6): string => {
    if (value.length <= startLength + endLength) return value; // No need to shorten
    return `${value.slice(0, startLength)}...${value.slice(-endLength)}`;
};

export default shortenBlockId;