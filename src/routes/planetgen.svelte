<script>
	import Frame from '$lib/Frame.svelte';
	import Navbar from '$lib/Navbar.svelte';
</script>

<svelte:head>
	<title>Planet Generator</title>
	<script
		defer
		src="https://cdn.jsdelivr.net/gh/believeinlain/planetgen-js@0.1.3/dist/planetgen.js"></script>
</svelte:head>

<Frame type="header">
	<Navbar title="Planet Generator" />
</Frame>

<canvas id="renderCanvas" touch-action="none" />

<Frame type="footer">
	<p>
		Applet by Stephanie Aelmore. Source available
		<a href="https://github.com/believeinlain/planetgen-js">here</a>
		under the MIT License.
	</p>
</Frame>

<style>
	canvas {
		width: 100%;
		height: 100%;
		top: 0;
		bottom: 0;
		left: 0;
		right: 0;

		position: absolute;
	}
</style>
