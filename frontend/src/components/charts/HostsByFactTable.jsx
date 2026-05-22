import { onMount } from 'solid-js'
import { getCoreRowModel, createSolidTable } from '@tanstack/solid-table';
import { createResource, For } from 'solid-js';
import { useParams, A } from "@solidjs/router";
import { LegendTable } from '../common/Tables';
import { useContextMessage } from "../common/MessageProvider";
import JsonMessage from "../common/JsonMessage";
import { factToFriendlyName } from "../common/Util";


export default function HostsByFactsTable() {
  
  const getApiUrl = () =>  {
    var url = '/api/hosts/groupby/fact';
    if (! (typeof useParams().fkey === 'undefined')) {
      url += '/' + encodeURIComponent(useParams().fkey);
    }
    if (! (typeof useParams().fvalue === 'undefined')) {
      url += '/' + encodeURIComponent(useParams().fvalue);
    }
    return url;
  }
  

  const apiFetch = async () => {
    const f = await fetch(getApiUrl());
    const j = await f.json();
    return {fetch: f, json: j};
  }
  const fkey = () => useParams().fkey;
  const factFriendlyName = () => factToFriendlyName(fkey());
  const [apiUrl, setApiUrl] = createResource(fkey, getApiUrl);
  const [apiResource] = createResource(apiUrl, apiFetch);

  const columns = () => [
    { accessorKey: 'facts_key', header: factFriendlyName(), cell: v => v.getValue(), class: 'text-align-left', footer: 'Total' },
    { accessorKey: 'count', header: 'Hosts', cell: v => v.getValue(), class: 'text-align-right ',
      footer: ({table}) => table.getFilteredRowModel().rows.reduce((total, row) => total + row.getValue('count'), 0),
    },
  ];

  const tableOptions = () => {
    return {
      data: apiResource().json.result,
      columns: columns(),
      getCoreRowModel: getCoreRowModel(),
    };
  }

  return (
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
      <Match when={ apiResource.state === 'ready' && apiResource().fetch.status == 200  }>
        <div class="report">
          <LegendTable options={tableOptions()} showHeader={true}/>
        </div>
      </Match>
    </Switch>
  )
}
