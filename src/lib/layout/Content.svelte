<div class="content">
  <slot />
</div>

<style>
  .content {
    background-color: rgba(17, 17, 17, 0.8);
    max-width: 800px;
    margin: auto;
    padding: 2%;
    padding-top: calc(var(--header-height) - 10px);
  }
</style>
