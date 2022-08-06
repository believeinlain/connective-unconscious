<script>
  import { session } from '$app/stores';
  import DropdownItem from './DropdownItem.svelte';
  import Dropdown from './Dropdown.svelte';
  export let title;
</script>

<h1>{title}</h1>
<div class="navbar">
  <Dropdown link="/" label="Home" />
  <Dropdown link="/gallery" label="Gallery">
    <DropdownItem link="/gallery/nihonnoyume" label="日本の夢" />
    <DropdownItem link="/gallery/jikannoyume" label="時間の夢" />
  </Dropdown>
  <Dropdown link="/demos" label="Demos">
    <DropdownItem link="/demos/planetgen" label="🪐" />
  </Dropdown>
  <Dropdown link="/resume" label="Resume" />
  {#if $session.user}
    <Dropdown link="/logout" label="Logout" />
  {/if}
</div>

<style>
  .navbar {
    display: inline-flex;
    /* grid-template-columns: 10% 10%; */
    /* flex-direction: row; */
    height: 100%;
    margin: auto;
  }
</style>
