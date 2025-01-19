// Gestion de la sélection interactive du pays

const resultsContainer = document.getElementById("autocomplete-results");
const countryInput = document.getElementById("countryInput");

countryInput.addEventListener("input", searchAbbreviation);
function searchAbbreviation() {
    const query = countryInput.value.toLowerCase();
    resultsContainer.innerHTML = ""; // Clear previous results

    if (query) {
        const filteredOptions = Object.keys(countriesAbbreviation).filter(option => 
            option.toLowerCase().includes(query)
        );

        filteredOptions.forEach(option => {
            const resultItem = document.createElement("div");
            resultItem.textContent = option;
            resultItem.addEventListener("click", function() {
                countryInput.value = option; // Set input value to the clicked option
                resultsContainer.innerHTML = ""; // Clear results
            });
            resultsContainer.appendChild(resultItem);
        });
    }
}

document.addEventListener("click", function(event) {
    if (!event.target.closest(".autocomplete-container")) {
        resultsContainer.innerHTML = ""; // Close dropdown if clicked outside
    }
});