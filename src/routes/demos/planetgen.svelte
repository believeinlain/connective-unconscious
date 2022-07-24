<script>
  import Page from '$lib/Page.svelte';
</script>

<svelte:head>
  <script
    defer
    src="https://cdn.jsdelivr.net/gh/believeinlain/planetgen-js@0.1.3/dist/planetgen.js"></script>
</svelte:head>

<Page page_title="Planet Generator">
  <span>
    <canvas id="renderCanvas" touch-action="none" />
  </span>
  <span slot="footer">
    <p>
      Applet by Stephanie Aelmore. Source available
      <a href="https://github.com/believeinlain/planetgen-js" target="_blank"
        >here</a>
      under the MIT License.
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
