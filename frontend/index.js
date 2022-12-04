function main() {
    const fileInput = document.getElementById('upload');
    const fileDropArea = document.getElementById('upload-label');

    if (fileInput) {
        document.addEventListener('paste', async (event) => {
            let files = event.clipboardData.files;

            if (files[0]) {
                if (files[0].type.startsWith('image/')) {
                    fileInput.files = files;
                    fileInput.classList.add("is-valid");

                    if (fileDropArea) {
                        fileDropArea.classList.add("upload-focused");
                        await new Promise(resolve => setTimeout(resolve, 400));
                        fileDropArea.classList.remove("upload-focused");
                    }
                }
            }
        });

        fileInput.addEventListener('change', () => {
            fileInput.classList.add("is-valid");
        });
    }

    if (fileDropArea) {
        fileDropArea.addEventListener('dragover', (event) => {
            event.preventDefault();
            fileDropArea.classList.add('upload-hovered');
        });

        fileDropArea.addEventListener('dragenter', () => {
            fileDropArea.classList.add('upload-hovered');
        });

        fileDropArea.addEventListener('dragleave', () => {
            fileDropArea.classList.remove('upload-hovered');
        });

        fileDropArea.addEventListener('drop', (event) => {
            event.preventDefault();
            fileDropArea.classList.remove('upload-hovered');

            let files = event.dataTransfer.files;
            if (files[0]) {
                if (files[0].type.startsWith('image/')) {
                    fileInput.files = files;
                    fileInput.classList.add("is-valid");
                }
            }
        });
    }

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
        output.innerHTML =
            `<div class="lg-shadow" id="output-placeholder">Output Image</div>`;
    }
}

main();