<script>
  import Frame from '$lib/Frame.svelte';
  import Navbar from '$lib/Navbar.svelte';
  import Content from '$lib/Content.svelte';

  // Title used for the html head element and for the navbar header
  export let page_title;

  // If set to false, hide the footer rather than showing the default
  // copyright text
  export let show_footer = true;

  const head_title = page_title
    ? `${page_title} : Connective Unconscious`
    : 'Connective Unconscious';
  const display_title = page_title ? page_title : 'Connective Unconscious';
</script>

<svelte:head>
  <title>{head_title}</title>
</svelte:head>

<Frame type="header">
  <Navbar title={display_title} />
</Frame>

<Content>
  <!-- Page content goes here -->
  <slot />
</Content>

{#if show_footer == true}
  <Frame type="footer">
    <slot name="footer">
      <!-- Default copyright message -->
      <p>
        Website by Stephanie Aelmore. Source available
        <a href="https://github.com/believeinlain/connective-unconscious"
          >here</a
        >
        under the MIT License.
      </p>
    </slot>
  </Frame>
{/if}
