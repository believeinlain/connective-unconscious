<svelte:head>
    <title>時間の夢</title>
</svelte:head>

<script>
    import Frame from "$lib/Frame.svelte";
    import Navbar from "$lib/Navbar.svelte";
    import Gallery from "$lib/Gallery.svelte";
</script>

<Frame type='header'>
    <Navbar title='時間の夢'></Navbar>
</Frame>

<Gallery album_name='jikannoyume'></Gallery>

<Frame type='footer'>
    <p>Photos by Stephanie Aelmore, no rights reserved.</p>
</Frame>
