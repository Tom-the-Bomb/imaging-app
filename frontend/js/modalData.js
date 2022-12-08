const legoInput = `
<div class="form-floating">
    <input type="number" class="form-control" id="size" value="40">
    <label for="size">Brick amount</label>
</div>
`

const mcInput = `
<div class="form-floating">
    <input type="number" class="custom-input form-control" id="size" value="70">
    <label for="size">Block amount</label>
</div>
`

const brailleInput = `
<div class="form-check form-switch">
    <input class="form-check-input" type="checkbox" id="invert">
    <label class="form-check-label" for="invert">Inverted</label>
</div>
<div class="form-floating">
    <input type="number" class="form-control" id="threshold" value="90">
    <label for="threshold">threshold</label>
</div>
<div class="form-floating">
    <input type="number" class="form-control" id="size" value="130">
    <label for="size">Character count</label>
</div>
`

const asciiInput = `
<div class="form-check form-switch">
    <input class="form-check-input" type="checkbox" id="invert">
    <label class="form-check-label" for="invert">Inverted</label>
</div>
<div class="form-floating">
    <input type="number" class="form-control" id="size" value="130">
    <label for="size">Character count</label>
</div>
`

const matrixInput = `
<div class="form-check form-switch">
    <input class="form-check-input" type="checkbox" id="num_only">
    <label class="form-check-label" for="num_only">Numbers only</label>
</div>
<div class="form-floating">
    <input type="number" class="form-control" id="size" value="80">
    <label for="size">Character count</label>
</div>
`

const shapesInput = `
<div class="form-check form-switch">
    <input class="form-check-input" type="checkbox" id="gif" checked>
    <label class="form-check-label" for="gif">Animated</label>
</div>
<div class="form-floating">
    <input type="number" class="form-control" id="density" value="10000">
    <label for="density">Shape amount</label>
</div>
<div class="form-floating">
    <input type="number" class="form-control" id="block" value="10">
    <label for="block">Size of shape</label>
</div>
`

const formMapping = {
    'lego': legoInput,
    'minecraft': mcInput,
    'paint': 'No options available',
    'frost': 'No options available',
    'braille': brailleInput,
    'ascii': asciiInput,
    'matrix': matrixInput,
    'lines': shapesInput,
    'balls': shapesInput,
    'squares': shapesInput,
}