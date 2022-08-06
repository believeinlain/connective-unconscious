<script>
    export let link;
    export let label;
    
    let hovering = false;

    function enter() {
      hovering = true;
    }

    function leave() {
      hovering = false;
    }
</script>

<div class="menu" on:focus={null} on:mouseover={enter} on:mouseleave={leave}>
  <a class="link" href={link}>{label}</a>
  <div class="dropdown" class:hovering={hovering}>
    <a class="link" href={link}>{label}</a>
    <slot/>
  </div>
</div>

<style>
    .menu {
      flex-direction: column;
      display: flex;
    }
    .dropdown {
      display: none;
    }
    .hovering {
      position: absolute;
      display: grid;
    }
    .link {
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