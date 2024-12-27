function validatePageNumber(value, totalPages) {
    const page = Number(value);

    const currentUrl = window.location.pathname;
    const firstPage = currentUrl == "/checkpoint" ? 0 : 1;

    if (page >= firstPage && page <= totalPages) {
        window.location.href = `?p=${page}`;
    } else {
        showAlert(`Please enter values between ${firstPage} and ${totalPages}`);
    }
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