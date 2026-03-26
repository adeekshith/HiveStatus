let dataFetchInterval;
let resizeObserver;

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
        // Optionally add an icon or styling class here if needed, 
        // but CSS handles the red color.
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
        ul.dataset.items = groupServices.length;

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

    layoutAllGrids();
}

function layoutHoneycomb(gridContainer) {
    const hexagons = Array.from(gridContainer.querySelectorAll(':scope > .hexagon'));
    if (hexagons.length === 0) return;

    const style = getComputedStyle(document.documentElement);
    const hexWidth = parseFloat(style.getPropertyValue('--hex-width'));
    const hexMarginX = parseFloat(style.getPropertyValue('--hex-margin-x'));
    const cellWidth = hexWidth + 2 * hexMarginX;
    const containerWidth = gridContainer.clientWidth;
    const perRow = Math.max(2, Math.floor(containerWidth / cellWidth));

    // Clear existing rows
    gridContainer.innerHTML = '';

    let i = 0;
    let isOffset = false;
    while (i < hexagons.length) {
        const row = document.createElement('div');
        row.className = isOffset ? 'hex-row hex-row-offset' : 'hex-row';
        const count = isOffset ? Math.min(perRow - 1, hexagons.length - i) : Math.min(perRow, hexagons.length - i);
        for (let j = 0; j < count; j++) {
            row.appendChild(hexagons[i++]);
        }
        gridContainer.appendChild(row);
        isOffset = !isOffset;
    }
}

function layoutAllGrids() {
    document.querySelectorAll('.hexagon-grid-container').forEach(layoutHoneycomb);
}

function setupResizeObserver() {
    if (resizeObserver) resizeObserver.disconnect();
    resizeObserver = new ResizeObserver(() => {
        layoutAllGrids();
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