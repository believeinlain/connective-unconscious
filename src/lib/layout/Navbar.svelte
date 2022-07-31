<script>
  import { session } from '$app/stores';
  export let title;
</script>

<h1>{title}</h1>
<div class="navbar">
  <a href="/">Home</a>
  <a href="/gallery">Gallery</a>
  <a href="/demos">Demos</a>
  <a href="/resume">Resume</a>
  {#if $session.user}
    <a href="/logout">Logout</a>
  {/if}
</div>

<style>
  .navbar {
    display: flex;
    flex-direction: row;
    height: 100%;
    margin: auto;
  }
  .navbar a {
    display: block;
    float: left;
    width: fit-content;
    height: fit-content;
    padding: 15px;
    margin: 2px;
    border: 1px solid #222;
    background-color: rgba(34, 34, 34, 0.8);
    text-decoration: none;
  }
  a:link {
    border-color: #888;
    color: #eee;
  }
  a:visited {
    border-color: #555;
    color: #ddd;
  }
  a:hover {
    color: #fff;
    border-color: #fff;
    text-decoration: none;
  }
</style>
