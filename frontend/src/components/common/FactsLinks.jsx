import { createResource } from 'solid-js';
import { A } from "@solidjs/router";
import { useContextMessage } from "./MessageProvider";

import JsonMessage from "./JsonMessage";


export default function FactsLinks() { 
  // const [message, setMessage] = useContextMessage();

  const apiFetch = async () => {
    const f = await fetch('/api/hosts/facts');
    const j = await f.json();
    return {fetch: f, json: j};
  }

  // const factsApiFetch = async () => await fetch('/api/hosts/facts');
  // const factsApiJson = async (f) => await f.json();

  const [apiResource] = createResource(apiFetch);
  // const [factsJsonResource] = createResource(factsApiResource, factsApiJson)

  const facts = () => {
    return apiResource().json.result;
  }

  return (
    <Switch fallback={<div>Not Found</div>}>
      <Match when={apiResource.state === 'pending' }>
        <div class="loading">Loading...</div>
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
      <div class="links">
        <For each={facts()}>{(fact, i) =>
          <>
              <A activeClass="active" end="true" class="mini"
                href={'/facts/' + encodeURIComponent(fact.name)}
                >{fact.description}</A>{' '}
          </>
        }</For>
      </div>
      </Match>
    </Switch>
  )
}
