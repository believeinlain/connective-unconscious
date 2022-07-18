
<svelte:head>
    <title>Planet Generator</title>
    <script defer src="https://cdn.jsdelivr.net/gh/believeinlain/planetgen-js@0.1.3/dist/planetgen.js"></script>
</svelte:head>

<canvas id="renderCanvas" touch-action="none"></canvas>

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

