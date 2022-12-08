const legoInput = `
<div class="form-floating">
    <input type="number" class="form-control" id="size" value="40" min="1" max="200">
    <div class="invalid-feedback">
        Value must be an integer between 1 and 200
    </div>
    <label for="size">Brick amount</label>
</div>
`

const mcInput = `
<div class="form-floating">
    <input type="number" class="custom-input form-control" id="size" value="70" min="1" max="200">
    <div class="invalid-feedback">
        Value must be an integer between 1 and 200
    </div>
    <label for="size">Block amount</label>
</div>
`

const paintInput = `
<div class="form-floating">
    <input type="number" class="custom-input form-control" id="radius" value="5" min="1" max="20">
    <div class="invalid-feedback">
        Value must be an integer between 1 and 20
    </div>
    <label for="radius">Radius of paint particle</label>
</div>
<div class="form-floating">
    <input type="number" class="custom-input form-control" id="intensity" value="60" min="1" max="100" step="0.01">
    <div class="invalid-feedback">
        Value must be a number between 1 and 100 (with step of 0.01)
    </div>
    <label for="intensity">Intensity</label>
</div>
`

const brailleInput = `
<div class="form-check form-switch">
    <input class="form-check-input" type="checkbox" id="invert">
    <label class="form-check-label" for="invert">Inverted</label>
</div>
<div class="form-floating">
    <input type="number" class="form-control" id="threshold" value="90" min="0" max="255">
    <div class="invalid-feedback">
        Value must be an integer between 0 and 255
    </div>
    <label for="threshold">threshold</label>
</div>
<div class="form-floating">
    <input type="number" class="form-control" id="size" value="130" min="1" max="200">
    <div class="invalid-feedback">
        Value must be an integer between 1 and 200
    </div>
    <label for="size">Character count</label>
</div>
`

const asciiInput = `
<div class="form-check form-switch">
    <input class="form-check-input" type="checkbox" id="invert">
    <label class="form-check-label" for="invert">Inverted</label>
</div>
<div class="form-floating">
    <input type="number" class="form-control" id="size" value="130" min="1" max="200">
    <div class="invalid-feedback">
        Value must be an integer between 1 and 200
    </div>
    <label for="size">Character count</label>
</div>
`

const matrixInput = `
<div class="form-check form-switch">
    <input class="form-check-input" type="checkbox" id="num_only">
    <label class="form-check-label" for="num_only">Numbers only</label>
</div>
<div class="form-floating">
    <input type="number" class="form-control" id="size" value="80" min="1" max="200">
    <div class="invalid-feedback">
        Value must be an integer between 1 and 200
    </div>
    <label for="size">Character count</label>
</div>
`

const shapesInput = `
<div class="form-check form-switch">
    <input class="form-check-input" type="checkbox" id="gif" checked>
    <label class="form-check-label" for="gif">Animated</label>
</div>
<div class="form-floating">
    <input type="number" class="form-control" id="density" value="10000" min="1" max="20000">
    <div class="invalid-feedback">
        Value must be an integer between 1 and 20000
    </div>
    <label for="density">Shape amount</label>
</div>
<div class="form-floating">
    <input type="number" class="form-control" id="block" value="10" min="1" max="50">
    <div class="invalid-feedback">
        Value must be an integer between 1 and 50
    </div>
    <label for="block">Size of shape</label>
</div>
`

const formMapping = {
    'lego': legoInput,
    'minecraft': mcInput,
    'paint': paintInput,
    'frost': 'No options available',
    'braille': brailleInput,
    'ascii': asciiInput,
    'matrix': matrixInput,
    'lines': shapesInput,
    'balls': shapesInput,
    'squares': shapesInput,
}