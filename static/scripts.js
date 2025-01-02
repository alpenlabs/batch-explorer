function validatePageNumber(value, totalPages) {
    console.log("validatePageNumber called with:", value, totalPages); // Debugging log
    const page = Number(value);

    const currentUrl = window.location.pathname;
    const firstPage = currentUrl === "/checkpoint" ? 0 : 1;

    if (page >= firstPage && page <= totalPages) {
        const url = `?p=${page}`;
        console.log("Valid page number. Loading content for URL:", url); // Debugging log
        loadContent(url, "#dynamicContent"); // Dynamically load the content
    } else {
        showAlert(`Please enter values between ${firstPage} and ${totalPages}`);
    }
}

function attachPageInputEventListeners() {
    const pageInput = document.querySelector(".pageInput");
    console.log("Page input element:", pageInput); // Debugging log
    if (pageInput) {
        pageInput.removeEventListener("keydown", handlePageInputKeyDown);
        pageInput.removeEventListener("blur", handlePageInputBlur);

        pageInput.addEventListener("keydown", handlePageInputKeyDown);
        pageInput.addEventListener("blur", handlePageInputBlur);
        console.log("Event listeners attached to page input."); // Debugging log
    } else {
        console.error("Page input element not found.");
    }
}

function handlePageInputKeyDown(event) {
    console.log("Keydown event triggered:", event.key); // Debugging log
    if (event.key === "Enter") {
        const totalPages = Number(event.target.getAttribute("data-total-pages"));
        validatePageNumber(event.target.value, totalPages);
    }
}

function handlePageInputBlur(event) {
    console.log("Blur event triggered."); // Debugging log
    const totalPages = Number(event.target.getAttribute("data-total-pages"));
    validatePageNumber(event.target.value, totalPages);
}

let isMenuOpen = false;

const toggleMenu = () => {
    const menuWrapper = document.querySelector(".navbarMenuWrapper");
    menuWrapper.classList.toggle("showMenu");

    // Update the isMenuOpen state
    isMenuOpen = menuWrapper.classList.contains("showMenu");
};

// Custom Alert Function
const showAlert = (message) => {
    const alertBox = document.createElement("div");
    alertBox.className = "customAlert";
    alertBox.textContent = message;

    document.body.appendChild(alertBox);

    setTimeout(() => {
        alertBox.remove();
    }, 3000); // Remove the alert after 3 seconds
};

function loadContent(url, targetId) {
    fetch(url)
        .then((response) => response.text())
        .then((html) => {
            const parser = new DOMParser();
            const doc = parser.parseFromString(html, "text/html");

            // Update the target container
            const newContent = doc.querySelector(targetId)?.innerHTML;
            if (newContent) {
                document.querySelector(targetId).innerHTML = newContent;
            }

            // Reattach pagination event listeners
            const newPagination = doc.querySelector(".pagination")?.innerHTML;
            if (newPagination) {
                document.querySelector(".pagination").innerHTML = newPagination;
                attachPaginationEventListeners();
                attachPageInputEventListeners(); // Reattach input field listeners
            }

            // Reattach dynamic link listeners
            attachDynamicLinkEventListeners();

            // Update the browser's URL
            history.pushState(null, "", url);
        })
        .catch((error) => console.error("Error loading content:", error));
}

// Attach event listeners for dynamic links
function attachDynamicLinkEventListeners() {
    // Select all elements with the `data-link` attribute
    document.querySelectorAll("[data-link]").forEach((link) => {
        // Remove any existing event listeners to prevent duplication
        link.removeEventListener("click", handleDynamicLinkClick);

        // Add a new event listener
        link.addEventListener("click", handleDynamicLinkClick);
    });
}

// Handle dynamic link click
function handleDynamicLinkClick(event) {
    event.preventDefault();

    const url = this.getAttribute("href");
    const targetId = this.getAttribute("data-target") || "#dynamicContent";

    loadContent(url, targetId);
}

// Attach event listeners for pagination buttons
function attachPaginationEventListeners() {
    // Select all pagination buttons
    document.querySelectorAll(".pageButton").forEach((button) => {
        // Remove any existing event listeners to prevent duplication
        button.removeEventListener("click", handlePaginationClick);

        // Add a new event listener
        button.addEventListener("click", handlePaginationClick);
    });
}

// Handle pagination button click
function handlePaginationClick(event) {
    event.preventDefault();

    const page = this.getAttribute("data-page");
    const url = `?p=${page}`;
    loadContent(url, "#dynamicContent");
}

// Handle back/forward navigation
window.addEventListener("popstate", () => {
    const url = window.location.href;
    loadContent(url, "#dynamicContent");
});

// Initial setup
attachDynamicLinkEventListeners();
attachPaginationEventListeners();