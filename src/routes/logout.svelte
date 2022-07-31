<script>
  import Page from '$lib/layout/Page.svelte';
  import { browser } from '$app/env';
  import { session } from '$app/stores';

  if (browser) {
    session.update( (session) => {session.user = undefined; return session;});
  }
</script>

<Page>
  <span>
    {#if $session.user}
      <p>Logging out user: {$session.user}...</p>
    {:else}
      <p>User logged out successfully.</p>
    {/if}
  </span>
</Page>
