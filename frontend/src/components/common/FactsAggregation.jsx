import { createEffect, createResource, createSignal } from 'solid-js';
import { useParams, A } from "@solidjs/router";
import { useContextMessage } from "./MessageProvider";
import { ReportTable } from "./Tables";
import JsonMessage from "./JsonMessage";
import { capitalize } from "./Util";
import FactsByEnvironmentChart from "../charts/FactsByEnvironmentChart";
import CPUFactsByEnvironmentChart from "../charts/CPUFactsByEnvironmentChart";
import HostsByFactTable from "../charts/HostsByFactTable";
import CPUFactsHistoryChart from "../charts/CPUFactsHistoryChart";

import {
  getCoreRowModel,
} from '@tanstack/solid-table';

export default function FactsAggregation() { 
  const [message, setMessage] = useContextMessage();

  const getApiUrl = () =>  {
    var url = '/api/hosts/facts';
    if (! (typeof useParams().fkey === 'undefined')) {
      url += '/' + encodeURIComponent(useParams().fkey);
    }
    return url;
  }
  
  const fkey = () => useParams().fkey;
  const [apiUrl, setApiUrl] = createResource(fkey, getApiUrl);

  const apiFetch = async () => {
    const f = await fetch(apiUrl());
    const j = await f.json();
    return {fetch: f, json: j};
  }

  const [apiResource, {mutate, refetch}] = createResource(apiUrl, apiFetch);
  

  return (
    <>
      <Switch>
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
        <Match when={ apiResource.state === 'ready' && apiResource().fetch.status == 200 }>
          <Show when={ apiResource().json.result[0].aggregable === true }>
            <div>
            <FactsByEnvironmentChart />
            <HostsByFactTable />
            </div>
            <div>
            <Show when={ apiResource().json.result[0].online_cpu === true }>
              <CPUFactsByEnvironmentChart />
              <CPUFactsHistoryChart />
            </Show>
            </div>
          </Show>
        </Match>
      </Switch>
    </>
  )
}
