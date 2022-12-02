function main() {
    const fnSelect = document.getElementById('function-select');
    const fileInput = document.getElementById('upload');

    if (fileInput) {
        fileInput.addEventListener('change', async () => {
            let file = fileInput.files[0];
            if (file) {
                let bytes = await file.arrayBuffer();
                
                if (fnSelect) {
                    let selected = fnSelect.options[fnSelect.selectedIndex];
                    if (selected.value === 'Select a function') {
                        return alert('Select a function');
                    }
                    await makeRequest(selected.value, bytes);
                } else {
                    return alert('Something went wrong');
                }
            }
        }, false)
    }
}

async function makeRequest(endpoint, bytes) {
    let data = new FormData();
    data.append('image', bytes);
    let payload = {
        method: 'POST',
        body: data,
    };

    let response = await fetch(`/${endpoint}`, payload);
    if (response.ok) {
        const output = document.getElementById('output-container');
        let url = URL.createObjectURL(
            await response.blob()
        );
        output.innerHTML = `<img id="output-image" src="${url}"/>`;
    } else {
        alert(`${response.status}: ${response.body}`);
    }
}

main();