import { createEffect, createResource, createSignal } from 'solid-js';
import { useParams, A } from "@solidjs/router";
import { useContextMessage } from "./MessageProvider";
import { ReportTable } from "./Tables";
import JsonMessage from "./JsonMessage";
import { capitalize, formatDate } from "./Util";
// import FactsByEnvironmentChart from "../charts/FactsByEnvironmentChart";
// import HostsByFactTable from "../charts/HostsByFactTable";
import FactsAggregation from "./FactsAggregation";

import {
  getCoreRowModel,
} from '@tanstack/solid-table';

export default function FactsTable() { 
  const [message, setMessage] = useContextMessage();

  const getApiUrl = () =>  {
    var url = '/api/report/facts';
    if (! (typeof useParams().fkey === 'undefined')) {
      url += '/' + encodeURIComponent(useParams().fkey);
    }
    if (! (typeof useParams().fvalue === 'undefined')) {
      url += '/' + encodeURIComponent(useParams().fvalue);
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

  // const apiFetch = async () => await fetch(apiUrl());
  // const apiJson = async (f) => await f.json();

  const [apiResource, {mutate, refetch}] = createResource(apiUrl, apiFetch);
  // const [jsonResource] = createResource(apiResource, apiJson);


  var columns = () => { 
    var c = [
      { accessorKey: 'hostname', header: v => capitalize(v.column.id), cell: v => v.getValue(), 
        class: 'text-align-left', footer: v => capitalize(v.column.id), },
      { accessorKey: 'bu', header: v => capitalize(v.column.id), cell: v => v.getValue(), 
        class: 'text-align-left', footer: v => capitalize(v.column.id), },
      { accessorKey: 'comment', header: () => 'Description', cell: v => v.getValue(), 
        class: 'text-align-left overflow', footer: () => 'Description', },
      { accessorKey: 'facts_datetime', header: 'Facts date', cell: v => formatDate(v.getValue()), 
        class: 'text-align-left', footer: 'Facts date', },
      { accessorKey: 'location', header: v => capitalize(v.column.id), cell: v => v.getValue(), 
        class: 'text-align-left', footer: v => capitalize(v.column.id), },
      { accessorKey: 'environment', header: v => capitalize(v.column.id), cell: v => v.getValue(), 
        class: 'text-align-left', footer: v => capitalize(v.column.id), },
    ];

    if (!(useParams().fkey === "os_version")) {
      c.push({ accessorKey: 'os_version', header: 'OS', cell: v => v.getValue(), 
        class: 'text-align-right small-width', footer: 'OS', });
    }
    c.push({ accessorKey: 'cpu', header: 'CPU', cell: v => v.getValue(), 
      class: 'text-align-right', footer: 'CPU', });
    c.push({ accessorKey: 'memorysize', header: 'Memory', cell: v => v.getValue(), 
      class: 'text-align-right', footer: 'Memory', });
    c.push({ accessorKey: 'dfsize', header: 'Disk', cell: v => v.getValue(), 
      class: 'text-align-right', footer: 'Disk', });
  
    c.push({ accessorKey: "facts_key", header: useParams().fkey, cell: v => v.getValue(), 
        class: 'text-align-left', footer: useParams().fkey, });
    return c;
  }

  const tableOptions = () => { 
    return {
      data: apiResource().json.result,
      columns: columns(),
      getCoreRowModel: getCoreRowModel(),
    };
  }

  return (
    <>
      {/* <Show when={useParams().fkey != 'facts_datetime'}>
        <FactsByEnvironmentChart />
        <HostsByFactTable />
      </Show> */}
      <FactsAggregation/>
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
          <div>
            <div class="links float-right">
            <a target="_self" href={getApiUrl() + '?accept=csv'}>Donwload CSV</a>
            </div>
            <div class="padding-top-30">
            <ReportTable options={tableOptions()}/>
            </div>
          </div>
        </Match>
      </Switch>
    </>
  )
}
