<script>
	import Frame from '$lib/Frame.svelte';
	import Navbar from '$lib/Navbar.svelte';
	import Gallery from '$lib/Gallery.svelte';
</script>

<svelte:head>
	<title>日本の夢</title>
</svelte:head>

<Frame type="header">
	<Navbar title="日本の夢" />
</Frame>

<Gallery album_name="nihonnoyume" />

<Frame type="footer">
	<p>Photos by Stephanie Aelmore, no rights reserved.</p>
</Frame>
