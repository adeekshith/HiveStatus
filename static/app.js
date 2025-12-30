const REFRESH_INTERVAL_MS = 60000;

async function fetchData() {
    try {
        const response = await fetch('/api/statuses');
        if (!response.ok) throw new Error('Network response was not ok');
        const data = await response.json();
        render(data);
        updateLastUpdated();
    } catch (error) {
        console.error('Error fetching data:', error);
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
}

function updateLastUpdated() {
    const el = document.getElementById('last-updated');
    const now = new Date();
    el.textContent = `Last updated: ${now.toLocaleTimeString()}`;
}

// Initial fetch and periodic refresh
fetchData();
setInterval(fetchData, REFRESH_INTERVAL_MS);
