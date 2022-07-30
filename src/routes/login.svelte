<script>
  import Page from '$lib/layout/Page.svelte';
  import { Form, Field } from "svelte-forms-lib";

  const formProps = {
    initialValues: {
      user: "",
      password: ""
    },
    validate: values => {
      let errs = {};
      if (values.user === "") {
        errs["user"] = "user is required";
      }
      if (values.password === "") {
        errs["password"] = "password is required";
      }
      return errs;
    },
    onSubmit: async values => {
      const response = await fetch('http://127.0.0.1:5173/login', {
        method: 'PUT',
        body: JSON.stringify({
          values
        })
		  });
      console.log("Values received from server:", response);
      result = response;
    }
  };

  export let pwd;
  console.log("Pwd received from server:", pwd);
  export let result = undefined;
  $: message = result ? "Success" : "Waiting";
</script>

<Page>
  <span>
    <h2>Please enter your credentials:</h2>
    <Form {...formProps}>
      <label for="user">User</label>
      <Field name="user" type="text" />

      <label for="password">Password</label>
      <Field name="password" type="password" />

      <button type="submit">submit</button>
      <p>{message}</p>
    </Form>
  </span>
</Page>
