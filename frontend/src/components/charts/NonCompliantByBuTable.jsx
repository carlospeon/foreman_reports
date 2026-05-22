import { onMount } from 'solid-js'
import { getCoreRowModel, createSolidTable } from '@tanstack/solid-table';
import { createResource, For } from 'solid-js';
import { useParams, A } from "@solidjs/router";
import { LegendTable } from '../common/Tables';
import { useContextMessage } from "../common/MessageProvider";
import JsonMessage from "../common/JsonMessage";
import { factToFriendlyName } from "../common/Util";


export default function NonCompliantByBuTable(props) {


  var apiUrl = '/api/hosts/erratas/groupby/bu';
  if (typeof(props.ts) !== 'undefined') {
    apiUrl = apiUrl + "/noncompliant/" + props.ts;
  }
  const apiFetch = async () => {
    const f = await fetch(apiUrl);
    const j = await f.json();
    return {fetch: f, json: j};
  }
  
  //const [apiUrl, setApiUrl] = createResource(fkey, getApiUrl);
  const [apiResource] = createResource(apiFetch);

  const columns = () => [
    { accessorKey: 'bu', header: 'BU', cell: v => v.getValue(), class: 'text-align-left', footer: 'Total' },
    { accessorKey: 'non_updated', header: 'Non Updated', cell: v => v.getValue(), class: 'text-align-right ',
      footer: ({table}) => table.getFilteredRowModel().rows.reduce((total, row) => total + row.getValue('non_updated'), 0), },
    { accessorKey: 'non_supported', header: 'Non Supported', cell: v => v.getValue(), class: 'text-align-right ',
      footer: ({table}) => table.getFilteredRowModel().rows.reduce((total, row) => total + row.getValue('non_supported'), 0), },
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
          <h3>Non Compliant</h3>
          <LegendTable options={tableOptions()} showHeader={true}/>
        </div>
      </Match>
    </Switch>
  )
}
