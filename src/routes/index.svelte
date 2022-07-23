<script>
	import Frame from '$lib/Frame.svelte';
	import Navbar from '$lib/Navbar.svelte';
</script>

<svelte:head>
	<title>Connective Unconscious</title>
</svelte:head>

<Frame type="header">
	<Navbar title="Connective Unconscious" />
</Frame>

<h1>Welcome to the Connective Unconscious</h1>
<p>This is where I host personal projects that I want to share.</p>
<ul>
	<li>
		<a href="/nihonnoyume/">日本の夢</a>
		An illustrated poem, half-remembered from disjointed dreams.
	</li>
	<li>
		<a href="/jikannoyume/">時間の夢</a>
		A collection of photographs.
	</li>
	<li>
		<a href="/planetgen/">🪐</a>
		A simple toy for generating faultlines, and eventually terrestrial planets.
	</li>
</ul>

<Frame type="footer">
	<p>
		Website by Stephanie Aelmore. Source available
		<a href="https://github.com/believeinlain/connective-unconscious">here</a>
		under the MIT License.
	</p>
</Frame>
