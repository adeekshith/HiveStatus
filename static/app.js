let dataFetchInterval;
let resizeObserver;
let resizeTimeout;

async function setupAndRun() {
    let refreshIntervalMs = 60000; // Default

    try {
        const response = await fetch('/api/config');
        if (!response.ok) throw new Error('Could not fetch config');
        const config = await response.json();

        console.log("Configuration loaded:", config); // For debugging
        document.title = config.page_title;

        if (config.refresh_interval_ms && config.refresh_interval_ms > 1000) { // Ensure interval is reasonable
            refreshIntervalMs = config.refresh_interval_ms;
        }

        // Set footer text from config
        const footerEl = document.getElementById('footer-powered');
        if (footerEl && config.footer_text) {
            footerEl.innerHTML = config.footer_text;
        }

    } catch (error) {
        console.error('Error setting up page, using defaults:', error);
        document.title = "HiveStatus"; // Fallback title
    }

    // Initial data fetch
    fetchData();

    // Setup periodic refresh
    if (dataFetchInterval) {
        clearInterval(dataFetchInterval);
    }
    dataFetchInterval = setInterval(fetchData, refreshIntervalMs);
}

async function fetchData() {
    try {
        const response = await fetch('/api/statuses');
        if (!response.ok) throw new Error('Network response was not ok');
        const data = await response.json();
        render(data);
        updateLastUpdated();
        clearError();
    } catch (error) {
        console.error('Error fetching data:', error);
        showError(`Connection lost: ${error.message}`);
    }
}

function showError(message) {
    const el = document.getElementById('footer-error');
    if (el) {
        el.textContent = message;
    }
}

function clearError() {
    const el = document.getElementById('footer-error');
    if (el) {
        el.textContent = '';
    }
}

function getGroupName(key) {
    if (!key || key.startsWith('_')) return "Ungrouped";
    const parts = key.split('_');
    if (parts.length > 0 && parts[0]) {
        return parts[0].charAt(0).toUpperCase() + parts[0].slice(1);
    }
    return "Ungrouped";
}

function render(services) {
    const container = document.getElementById('groups-container');
    container.innerHTML = '';

    const groups = {};
    services.forEach(service => {
        const groupName = getGroupName(service.key || service.name);
        if (!groups[groupName]) groups[groupName] = [];
        groups[groupName].push(service);
    });

    const sortedGroupNames = Object.keys(groups).sort((a, b) => {
        if (a === 'Ungrouped') return 1;
        if (b === 'Ungrouped') return -1;
        return a.localeCompare(b);
    });

    sortedGroupNames.forEach(groupName => {
        const groupServices = groups[groupName];
        const details = document.createElement('details');
        details.className = 'group-section';
        details.open = true;

        const summary = document.createElement('summary');
        summary.textContent = groupName;
        details.appendChild(summary);

        const ul = document.createElement('ul');
        ul.className = 'hexagon-grid-container';

        groupServices.forEach(service => {
            const result = service.results && service.results.length > 0 ? service.results[service.results.length - 1] : null;
            const isSuccess = result ? result.success : false;
            const colorClass = isSuccess ? 'hexagon-green' : 'hexagon-red';

            const li = document.createElement('li');
            li.className = `hexagon ${colorClass}`;

            const innerDiv = document.createElement('div');
            innerDiv.className = 'hexagon-inner';

            const nameSpan = document.createElement('span');
            nameSpan.className = 'hexagon-name';
            nameSpan.textContent = service.name;

            innerDiv.appendChild(nameSpan);
            li.appendChild(innerDiv);
            ul.appendChild(li);
        });

        details.appendChild(ul);
        container.appendChild(details);
    });

    // Defer honeycomb offset to after browser has laid out the flex-wrap items
    requestAnimationFrame(() => {
        requestAnimationFrame(() => {
            applyHoneycombOffsets();
        });
    });
}

/**
 * Detect visual rows from flex-wrap layout and apply honeycomb offset.
 * CSS flex-wrap handles all row wrapping; this only adds the half-hex
 * translateX to every other row for the beehive interlock effect.
 */
function applyHoneycombOffsets() {
    document.querySelectorAll('.hexagon-grid-container').forEach(grid => {
        const hexagons = Array.from(grid.querySelectorAll('.hexagon'));
        if (hexagons.length === 0) return;

        // Reset previous offsets
        hexagons.forEach(hex => {
            hex.classList.remove('hex-offset');
        });

        // Detect visual rows by comparing offsetTop
        const rows = [];
        let currentRowTop = -Infinity;
        let currentRow = [];

        hexagons.forEach(hex => {
            const top = hex.offsetTop;
            if (Math.abs(top - currentRowTop) > 10) {
                if (currentRow.length > 0) rows.push(currentRow);
                currentRow = [hex];
                currentRowTop = top;
            } else {
                currentRow.push(hex);
            }
        });
        if (currentRow.length > 0) rows.push(currentRow);

        // Only apply honeycomb offset if there are multiple rows
        if (rows.length > 1) {
            rows.forEach((row, i) => {
                if (i % 2 === 1) {
                    row.forEach(hex => hex.classList.add('hex-offset'));
                }
            });
        }
    });
}

function setupResizeObserver() {
    if (resizeObserver) resizeObserver.disconnect();
    resizeObserver = new ResizeObserver(() => {
        clearTimeout(resizeTimeout);
        resizeTimeout = setTimeout(() => {
            requestAnimationFrame(applyHoneycombOffsets);
        }, 100);
    });
    const app = document.getElementById('app');
    if (app) resizeObserver.observe(app);
}

function updateLastUpdated() {
    const el = document.getElementById('last-updated');
    const now = new Date();
    el.textContent = `Last updated: ${now.toLocaleTimeString()}`;
}

// Initial setup and periodic refresh
setupAndRun();
setupResizeObserver();
