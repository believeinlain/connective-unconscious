<script>
  export let title;
</script>

<h1>{title}</h1>
<div class="navbar">
  <a href="/">Home</a>
  <a href="/gallery">Gallery</a>
  <a href="/demos">Demos</a>
  <a href="/resume">Resume</a>
  <a href="/login">🔒</a>
</div>

<style>
  .navbar {
    display: flex;
    flex-direction: row;
    height: 100%;
    margin: auto;
  }
  a {
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
