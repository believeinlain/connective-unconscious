<script>
  import Page from '$lib/layout/Page.svelte';
</script>

<svelte:head>
  <script
    defer
    src="https://cdn.jsdelivr.net/gh/believeinlain/planetgen-js@0.1.4.1/dist/planetgen.js"></script>
</svelte:head>

<Page page_title="Planet Generator 🪐">
  <span>
    <canvas id="renderCanvas" touch-action="none" />
  </span>
  <span slot="footer">
    <p>
      Applet by Stephanie Aelmore. Source available at
      <a href="https://github.com/believeinlain/planetgen-js" target="_blank">
        https://github.com/believeinlain/planetgen-js
      </a>
    </p>
  </span>
</Page>

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
