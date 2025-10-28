function createFullUrlJs(host, path, params, scheme = 'https') {
    // 1. Combine scheme and host to create the base URL object
    // Note: The host should NOT include the scheme here.
    const baseUrl = `${scheme}://${host}`;
    const url = new URL(path, baseUrl);
    
    // 2. Add query parameters if provided
    if (params) {
        for (const [key, value] of Object.entries(params)) {
            // .searchParams.set() handles automatic encoding
            url.searchParams.set(key, value);
        }
    }
    
    // 3. The .href property returns the full, correctly formatted URL
    return url.href;
}

// --- Usage Example ---
const hostName = "api.myservice.com";
const endpointPath = "/v3/data";
const queryParameters = {
    user_id: '456', 
    filter: 'active_only',
    limit: 10
};

const url = createFullUrlJs(hostName, endpointPath, queryParameters);

console.log(`Created URL: **${url}**`);
// Output: https://api.myservice.com/v3/data?user_id=456&filter=active_only&limit=10
