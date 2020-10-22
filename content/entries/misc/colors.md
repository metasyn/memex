# specifics
over time, I feel like I keep coming back to the same colors when I'm working on various projects.
i'm jotting them down here for safe keeping; they're colors I like!

<style>
.color-box {
  width: 100px;
  height: 100px;
  margin: 5px;
}
</style>

| color | hex |
| ----- | --- |
| <div class='color-box' style="background-color: #72c4d0"></div> | #72c4d0 |
| <div class='color-box' style="background-color: #a5d6c3"></div> | #a5d6c3 |


# generalities

i wrote some code not that long ago that generates palettes based on sine waves. some of it
came from [Jim Bumgardener's website](https://krazydad.com/tutorials/makecolors.php) - which goves over the general
concept of using sine waves to generate different types of color palettes. this particular combination was pleasing to me:

<div>
  <canvas id="canvas"></canvas>
  <noscript>
		the canvas that is generated on this page uses javascript. if you are not using javascript, the canvas will not render.
  </noscript>
</div>

<script>
function byte2Hex(n) {
	const nybHexString = '0123456789ABCDEF';
	return String(nybHexString.substr((n >> 4) & 0x0F, 1)) + nybHexString.substr(n & 0x0F, 1);
}

// Helper - https://krazydad.com/tutorials/makecolors.php
function RGB2Color(r, g, b) {
	return `#${byte2Hex(r)}${byte2Hex(g)}${byte2Hex(b)}`;
}

// Helper - https://krazydad.com/tutorials/makecolors.php
function makeColorGradient(
frequency1, frequency2, frequency3,
 phase1, phase2, phase3,
 center = 128, width = 127, len = 50,
) {
	const colors = [];

	for (let i = 0; i < len; ++i) {
		const red = Math.sin(frequency1 * i + phase1) * width + center;
		const grn = Math.sin(frequency2 * i + phase2) * width + center;
		const blu = Math.sin(frequency3 * i + phase3) * width + center;
		colors.push(RGB2Color(red, grn, blu));
	}
	return colors
}

function makePastels() {
	return makeColorGradient(0.5, 0.5, 0.3, 0, 2, 4, 200, 50, 80);
}

// Draw a stripe
function makeStripe(ctx, start, height, i, colors) {
	const sideLength = start + height;
	const data = `
		M 0 ${start}
		v ${height}
		L ${sideLength} 0
		h -${height}
		Z
	`;

  ctx.fillStyle = colors[i];
  let stripe = new Path2D(data);
  ctx.fill(stripe);
}

// Draw a lot of stripes
function makeStripes(ctx, height, colors) {
	const total = window.innerHeight + window.innerWidth;
	for (let i = 0; i < total; i += height) {
		const colorIndex = (i / height) % colors.length;
		makeStripe(ctx, i, height, colorIndex, colors);
	}
}

function main() {
  let canvas = document.getElementById('canvas');
  canvas.width = 500;
  canvas.height = 1000;
  let ctx = canvas.getContext('2d');
  const pastels = makePastels();
  makeStripes(ctx, 100,  pastels);
}

document.addEventListener('DOMContentLoaded', function() {
  main()
}, false);

</script>
