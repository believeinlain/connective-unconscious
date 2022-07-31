<script>
  import Page from '$lib/layout/Page.svelte';
  import { createForm } from 'svelte-forms-lib';
  import { session } from '$app/stores';

  let message;

  const { form, handleChange, handleSubmit } = createForm({
    initialValues: {
      username: '',
      password: ''
    },
    validate: (values) => {
      let errs = {};
      if (values.username === '') {
        errs['username'] = 'username is required';
      }
      if (values.password === '') {
        errs['password'] = 'password is required';
      }
      return errs;
    },
    onSubmit: async ({ username, password }) => {
      const response = await fetch('login', {
        method: 'POST',
        body: JSON.stringify({ username, password }),
        headers: {
          accept: 'application/json',
          'content-type': 'application/json'
        }
      });

      let body = await response.json();
      message = body.message;
      session.update((session) => {
        session.user = body.user;
        return session;
      });
    }
  });
</script>

<Page>
  <span>
    <h2>Please enter your credentials:</h2>
    <form class="form" on:submit={handleSubmit}>
      <div for="username">Username</div>
      <input
        id="username"
        name="username"
        on:change={handleChange}
        bind:value={$form.username}
      />
      <p />
      <div for="password">Password</div>

      <input
        id="password"
        name="password"
        type="password"
        on:change={handleChange}
        bind:value={$form.password}
      />

      <p />
      <button type="submit">Submit</button>
      {#if message}
        <p>{message}</p>
      {/if}
    </form>
  </span>
</Page>

<style>
  input {
    max-width: 400px;
    width: 100%;
    padding: 12px;
    box-sizing: border-box;
    background: rgb(70, 70, 70);
    color: #fff;
  }
  button {
    display: block;
    padding: 10px 18px;
    min-width: 120px;
    cursor: pointer;
    background: rgb(207, 207, 207);
  }
  button:hover {
    background: #fff;
  }
</style>
