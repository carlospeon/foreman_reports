import { MessageProvider } from '../common/MessageProvider';
import Message from './Message';
import JsonMessage from "../common/JsonMessage";
import { createSignal, createResource } from "solid-js";
import { Navigate, useSearchParams } from "@solidjs/router";

export default function Login() {
  const [searchParams, setSearchParams] = useSearchParams();

  const apiFetch = async (data) => {
    const f = await fetch('/api/login', {
      method: "POST",
      body: JSON.stringify(data),
      headers: {
        "Content-Type": "application/json",
      }
    });
    const j = await f.json();
    return {fetch: f, json: j};
  };

  const [form, setForm] = createSignal();
  const [apiResource] = createResource(form, apiFetch);

  const handleSubmit = (event) => {
    event.preventDefault();
    const data = {
      username: event.currentTarget.elements.username.value,
      password: event.currentTarget.elements.password.value
    };
    setForm(data);
  };

  const go = searchParams.go;
  var goto = '/home';
  if (!(typeof go === 'undefined')) {
    goto = decodeURIComponent(go);
  }

  return (
    <MessageProvider>
      <div class="contents">
        <div id="login">
          <form name="login" id="login" onSubmit={handleSubmit}>
            <table id="login">
              {/* <thead><tr><th>Login</th></t></thead> */}
              <tbody><tr><td><input
                name="username"
                id="username"
                placeholder="username"
                required="true"
                autofocus
              /></td></tr>
              <tr><td><input
                type="password"
                name="password"
                id="password"
                placeholder="password"
                required="true"
              /></td></tr>
              <tr><td><button type="submit">Submit</button></td></tr>
            </tbody></table>
            <Message />
          </form>
          <Switch>
            <Match when={apiResource.state === 'pending' }>
              <table id="login">
                <tbody><tr><td>Logging...</td></tr></tbody>
              </table>
            </Match>
            <Match when={apiResource.state === 'errored'}>
              <JsonMessage message={{
                status: apiResource.state, 
                result: String(apiResource.error)
              }}/>
            </Match>
            <Match when={ apiResource.state === 'ready' && apiResource().fetch.status != 200 }>
              <JsonMessage message={{
                status: apiResource().fetch.status, 
                result: apiResource().json.result
              }}/>
            </Match>
            <Match when={ apiResource.state === 'ready' && apiResource().fetch.status == 200  }>
              <Navigate href={ goto }/>
            </Match>
          </Switch>
        </div>
      </div>
    </MessageProvider>
  );
}
