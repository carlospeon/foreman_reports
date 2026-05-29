import { onMount } from 'solid-js'
import { Bar } from 'solid-chartjs'
import { createResource, For } from 'solid-js';
import { useParams, A } from "@solidjs/router";
import { getCoreRowModel } from '@tanstack/solid-table';
import { useContextMessage } from "../common/MessageProvider";
import JsonMessage from "../common/JsonMessage";
import { LegendTable } from '../common/Tables';
import { createEffect } from 'solid-js';
import { registerChartPlugins, barOptions } from './chartConfig';

export default function CPUFactsByEnvironmentChart() {
  // const [message, setMessage] = useContextMessage();

  const getApiUrl = () =>  {
    var url = '/api/report/facts';
    if (! (typeof useParams().fkey === 'undefined')) {
      url += '/' + encodeURIComponent(useParams().fkey);
    }
    if (! (typeof useParams().fvalue === 'undefined')) {
      url += '/' + encodeURIComponent(useParams().fvalue);
    }
    return url + '/cpu/groupby/environment';
  }
  
  const fkey = () => useParams().fkey;
  const [apiUrl, setApiUrl] = createResource(fkey, getApiUrl);

  // const apiFetch = async () => await fetch(apiUrl());
  // const apiJson = async (f) => await f.json();
  const apiFetch = async () => {
    const f = await fetch(apiUrl());
    const j = await f.json();
    return {fetch: f, json: j};
  }


  const [apiResource, {mutate, refetch}] = createResource(apiUrl, apiFetch);
  // const [jsonResource] = createResource(apiResource, apiJson);


  var columns = [
    { accessorKey: 'environment', header: 'Environment', cell: v => v.getValue(), class: 'text-align-left', footer: 'Environment' },
    // { accessorKey: 'facts_key', header: 'facts_key', cell: v => v.getValue(), class: 'text-align-right', footer: 'facts_key' }, 
    { accessorKey: 'cpu', header: 'cpu', cell: v => v.getValue(), class: 'text-align-right', footer: 'cpu' },
  ];

  const tableOptions = () => {
    return {
      data: apiResource().json.result,
      columns: columns,
      getCoreRowModel: getCoreRowModel(),
    };
  }

  onMount(() => registerChartPlugins())

  const fallback = () => {
    return (<div><p>Chart is not available</p></div>)
  }

  const chartOptions = () => barOptions('Online CPUs', 'Online CPUs');


  var data = {};
  const chartData = () => {
    return {
        labels: apiResource().json.result.map(function(i) { return i.environment; }),
        datasets: [
            {
                label: 'CPUs',
                data: apiResource().json.result.map(function(i) { return i.cpu; }),
            },
        ],
    }
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
          <div style="height: 400px; width: 600px;">
            <Bar
              fallback={fallback()}
              data={chartData()}
              options={chartOptions()}
            />
          </div>
          {/* <div style="max-width: 900px;">
            <LegendTable options={tableOptions()} />
          </div> */}
        </div>
      </Match>
    </Switch>
  )
}
