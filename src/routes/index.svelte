<script>
  import Frame from '$lib/Frame.svelte';
  import Navbar from '$lib/Navbar.svelte';
  import Content from '$lib/Content.svelte';
</script>

<svelte:head>
  <title>Connective Unconscious</title>
</svelte:head>

<Frame type="header">
  <Navbar title="Connective Unconscious" />
</Frame>

<Content>
  <h1>Welcome to the Connective Unconscious</h1>
  <p>Feel free to check out the different pages I have here via the navbar.</p>
  <p>
    I haven't updated this site in a while but I think I'm going to update more frequently in the
    near future.
  </p>
  <p>Thank you for visiting!</p>
</Content>

<Frame type="footer">
  <p>
    Website by Stephanie Aelmore. Source available
    <a href="https://github.com/believeinlain/connective-unconscious">here</a>
    under the MIT License.
  </p>
</Frame>
