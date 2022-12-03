function main() {
    document.addEventListener('paste', (event) => {
        let files = event.clipboardData.files;
        if (files) {
            fileInput.files = files;
            fileInputHandler();
        }
    });

    doFormValidation();
}

function doFormValidation() {
    const forms = document.querySelectorAll('.needs-validation');
    Array.prototype.slice.call(forms)
        .forEach((form) => {
            form.addEventListener('submit', (event) => {
                if (!form.checkValidity()) {
                    event.preventDefault()
                    event.stopPropagation()
                }
                form.classList.add('was-validated')
            }, false)
        });
}

async function formHandler(event) {
    event.preventDefault();
    const fnSelect = document.getElementById('function-select');
    const fileInput = document.getElementById('upload');
    const file = fileInput.files[0];

    if (file) {
        if (fnSelect) {
            const selected =
                fnSelect.options[fnSelect.selectedIndex];
            await makeRequest(selected.value, file);
        } else {
            return alert('Something went wrong');
        }
    }
}

async function makeRequest(endpoint, bytes) {
    const output = document.getElementById('output-container');
    output.innerHTML =
        `<div class="loading"></div>
        <p id="processing">Processing image...</p>`;

    let data = new FormData();
    data.append('image', bytes);
    const payload = {
        method: 'POST',
        body: data,
    };

    let response = await fetch(`/${endpoint}`, payload);
    if (response.ok) {
        const url = URL.createObjectURL(
            await response.blob()
        );
        output.innerHTML =
            `<img class="lg-shadow" id="output-image" src="${url}" alt="output image"/>`;
    } else {
        alert(`${response.status}: Something went wrong`);
    }
}

main();