import { onMount } from 'solid-js'
import { getCoreRowModel, createSolidTable } from '@tanstack/solid-table';
import { createResource, For } from 'solid-js';
import { useParams, A } from "@solidjs/router";
import { LegendTable } from '../common/Tables';
import { useContextMessage } from "../common/MessageProvider";
import JsonMessage from "../common/JsonMessage";
import { numberWithCommas } from "../common/Util";


export default function HostsResourcesByLocationTable() {
  
  const url = '/api/hosts/resources/groupby/location';
  

  const apiFetch = async () => {
    const f = await fetch(url);
    const j = await f.json();
    return {fetch: f, json: j};
  }

  const [apiResource] = createResource(apiFetch);

  const locale = navigator.language;
  const columns = () => [
    { accessorKey: 'location', header: 'Location', cell: v => v.getValue(), class: 'text-align-left', footer: 'Total' },
    { accessorKey: 'cpu', header: 'CPUs', cell: v => v.getValue() == null ? '' : (v.getValue()).toLocaleString(locale), class: 'text-align-right ',
      footer: ({table}) => (table.getFilteredRowModel().rows.reduce((total, row) => total + row.getValue('cpu'), 0)).toLocaleString(locale),
    },
    { accessorKey: 'memorysize', header: 'RAM (GB)', cell: v => v.getValue() == null ? '' : Number(v.getValue()).toLocaleString(locale), class: 'text-align-right ',
      footer: ({table}) => (table.getFilteredRowModel().rows.reduce((total, row) => total + Number(row.getValue('memorysize')), 0)).toLocaleString(locale),
    },
    { accessorKey: 'dfsize', header: 'Disk (GB)', cell: v => v.getValue() == null ? '' : Number(v.getValue()).toLocaleString(locale), class: 'text-align-right ',
      footer: ({table}) => (table.getFilteredRowModel().rows.reduce((total, row) => total + Number(row.getValue('dfsize')), 0)).toLocaleString(locale),
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
          <h3>Hosts Hardware resources by Location</h3>
          <LegendTable options={tableOptions()} showHeader={true}/>
        </div>
      </Match>
    </Switch>
  )
}
