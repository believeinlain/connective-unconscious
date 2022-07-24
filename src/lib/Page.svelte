<script>
  import Frame from '$lib/Frame.svelte';
  import Navbar from '$lib/Navbar.svelte';
  import Content from '$lib/Content.svelte';

  export let page_title;
  export let show_footer = true;
</script>

<svelte:head>
  <title>{page_title} : Connective Unconscious</title>
</svelte:head>

<Frame type="header">
  <Navbar title={page_title} />
</Frame>

<Content>
  <slot />
</Content>

{#if show_footer == true}
  <Frame type="footer">
    <slot name="footer">
      <!-- Default copyright message -->
      <p>
        Website by Stephanie Aelmore. Source available
        <a href="https://github.com/believeinlain/connective-unconscious">here</a>
        under the MIT License.
      </p>
    </slot>
  </Frame>
{/if}
