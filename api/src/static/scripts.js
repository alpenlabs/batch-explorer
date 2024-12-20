function validatePageNumber(value, totalPages) {
    const page = Number(value);
    if (page >= 1 && page <= totalPages) {
        window.location.href = `?page=${page}`;
    } else {
        alert(`Please enter a page number between 1 and ${totalPages}`);
    }
}
let isMenuOpen = false;

const toggleMenu = () => {
    const menuWrapper = document.querySelector(".navbarMenuWrapper");
    menuWrapper.classList.toggle("showMenu");
};