<script>
  import { session } from '$app/stores';
  export let type;
</script>

<div class={type}>
  <slot />
  {#if type == 'footer'}
    {#if $session.user}
      <p class="lock">{$session.user}🔓</p>
    {:else}
      <a class="lock" href="/login">🔒</a>
    {/if}
  {/if}
</div>

<style>
  .lock {
    text-decoration: none;
    align-self: center;
  }
  .header,
  .footer {
    box-sizing: border-box;
    margin: 0;
    position: fixed;
    left: 0;
    right: 0;
    width: 100%;
    background-color: rgba(17, 17, 17, 0.8);
    line-height: 0;
    padding: 10px;
    opacity: 0.8;
    z-index: 2;
  }
  .footer {
    min-height: 30px;
    bottom: 0;
    padding-top: 0;
    padding-bottom: 0;
    display: inline-flex;
    flex-direction: row;
    flex-wrap: nowrap;
    justify-content: space-between;
    align-items: baseline;
  }
  .header {
    top: 0;
    text-align: center;
    display: grid;
    height: var(--header-height);
  }
</style>
