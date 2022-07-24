<script>
  import Frame from '$lib/layout/Frame.svelte';
  import Navbar from '$lib/layout/Navbar.svelte';
  import Content from '$lib/layout/Content.svelte';

  // Title used for the html head element and for the navbar header
  export let page_title;

  const head_title = page_title
    ? `${page_title} : Connective Unconscious`
    : 'Connective Unconscious';
</script>

<svelte:head>
  <title>{head_title}</title>
</svelte:head>

<Frame type="header">
  <Navbar title={head_title} />
</Frame>

<Content>
  <!-- Page content goes here -->
  <slot />
</Content>

<br />

<Frame type="footer">
  <slot name="footer">
    <!-- Default copyright message -->
    <p>
      Website by Stephanie Aelmore. Source available
      <a
        href="https://github.com/believeinlain/connective-unconscious"
        target="_blank"
      >
        here
      </a>
      under the MIT License.
    </p>
  </slot>
</Frame>
