<script>
  import Page from '$lib/layout/Page.svelte';
  import { Form, Field, ErrorMessage } from 'svelte-forms-lib';
  import { session } from '$app/stores';

  let message;

  const formProps = {
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
  };
</script>

<Page>
  <span>
    <h2>Please enter your credentials:</h2>
    <Form {...formProps}>
      <label for="username">User</label>
      <Field name="username" type="text" />
      <ErrorMessage name="username" />

      <label for="password">Password</label>
      <Field name="password" type="password" />
      <ErrorMessage name="password" />

      <button type="submit">submit</button>
      {#if message}
        <p>{message}</p>
      {/if}
    </Form>
  </span>
</Page>
