var query = {};

function main() {
    const fileInput = document.getElementById('upload');
    const fnSelect = document.getElementById('function-select');
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
            if (fileInput.files.length > 0) {
                fileInput.classList.add("is-valid");
            } else {
                fileInput.classList.remove("is-valid");
            }
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

    if (fnSelect) {
        fnSelect.addEventListener('change', () => {
            const modalBody = document.getElementById('modal-body');
            const modalFooter = document.getElementById('modal-footer');
            const selected =
                fnSelect.options[fnSelect.selectedIndex];
            const modalHTML = formMapping[selected.value.toLowerCase()];

            if (modalBody) {
                query = {};
                modalBody.innerHTML = modalHTML;
            }

            const submit = document.getElementById('modal-submit');
            if (modalFooter && !submit) {
                modalFooter.innerHTML +=
                    `<button
                        type="submit"
                        id="modal-submit"
                        class="btn btn-success"
                    >
                        Save changes
                    </button>`;
            }
        })
    }

    doFormValidation();
    enableTooltips();
}

function enableTooltips() {
    const tooltipTriggerList = [].slice.call(
        document.querySelectorAll('[data-bs-toggle="tooltip"]')
    );
    tooltipTriggerList.map(
        function (tooltipTriggerEl) {
            return new bootstrap.Tooltip(tooltipTriggerEl)
        }
    );

    const optionsButton = document.getElementById('options-btn');
    if (optionsButton) {
        new bootstrap.Tooltip(optionsButton);
    }
}

function doFormValidation() {
    const forms = document.querySelectorAll('.needs-validation');
    Array.prototype.slice.call(forms)
        .forEach((form) => {
            form.addEventListener('submit', (event) => {
                if (!form.checkValidity()) {
                    event.preventDefault();
                    event.stopPropagation();
                }
                form.classList.add('was-validated');
            }, false);
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

    let params = new URLSearchParams(query);
    let response = await fetch(`/${endpoint}?` + params, payload);
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

function modalFormHandler(event) {
    event.preventDefault();
    const modal = document.getElementById('options-modal');
    const modalBody = document.getElementById('modal-body');

    if (modal) {
        bootstrap.Modal.getInstance(modal)
            .hide();
    }

    if (modalBody) {
        query = {};
        const inputs = modalBody.getElementsByTagName('input');

        for (let input of inputs) {
            let value = input.type === 'checkbox'
                ? input.checked
                : input.value;
            query[input.id] = value;
        }
    }
}

main();