<div class="entry-heading">
  <h4><slot name="position" /></h4>
  <h4><slot name="dates" /></h4>
</div>
<hr />
<div class="entry-heading">
    <h4><slot name="location" /></h4>
    <h4><slot name="gpa" /></h4>
</div>
<ul>
  <slot name="bullet-list" />
</ul>

<style>
  .entry-heading {
    display: grid;
    grid-template-columns: 80% 20%;
    height: fit-content;
  }
  hr,
  h4 {
    margin: 0;
  }
  h4 {
    padding-top: 3pt;
  }
  /* Apply formatting to any list items entered in the slots */
  ul :global(li) {
    padding-top: 3pt;
  }
  ul {
    margin-top: 0;
    margin-bottom: 20pt;
  }
</style>
