<script>
    export let link;
    export let label;
</script>

<a class="dropdown-item" href={link}>{label}</a>

<style>
    .dropdown-item {
      float: left;
      padding: 15px;
      margin: 0px 2px;
      border: 1px solid #222;
      background-color: rgba(34, 34, 34, 0.8);
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