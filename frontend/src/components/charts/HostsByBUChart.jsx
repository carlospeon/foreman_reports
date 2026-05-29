import { onMount } from 'solid-js'
import { Doughnut } from 'solid-chartjs'
import { getCoreRowModel } from '@tanstack/solid-table';
import { createResource, For } from 'solid-js';
import { useContextMessage } from "../common/MessageProvider";
import JsonMessage from "../common/JsonMessage";
import { LegendTable } from '../common/Tables';
import { registerChartPlugins, doughnutOptions } from './chartConfig';

export default function HostByBUChart() {
  // const [message, setMessage] = useContextMessage();

  // const apiFetch = async () => await fetch('/api/hosts/groupby/admingroup');
  // const apiJson = async (f) => await f.json();

  const apiFetch = async () => {
    const f = await fetch('/api/hosts/groupby/bu');
    const j = await f.json();
    return {fetch: f, json: j};
  }

  const [apiResource] = createResource(apiFetch);
  // const [jsonResource] = createResource(apiResource, apiJson);


  var columns = [
    { accessorKey: 'bu', header: 'BU', cell: v => v.getValue(), class: 'text-align-left', footer: 'Total' },
    { accessorKey: 'count', header: 'Hosts', cell: v => v.getValue(), class: 'text-align-right',
      footer: ({table}) => table.getFilteredRowModel().rows.reduce((total, row) => total + row.getValue('count'), 0),
    },
  ];

  const tableOptions = ()  => {
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

  const chartOptions = () => doughnutOptions('Hosts by Bussines Unit');

  const chartData = () => {
    return {
        labels: apiResource().json.result.map(function(i) { return i.bu; }),
        datasets: [
            {
                label: 'Hosts',
                data: apiResource().json.result.map(function(i) { return i.count; }),
            },
        ],
    }
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
        <div class="report">
          <div style="height: 400px; width: 300px;">
            <Doughnut
              fallback={fallback()}
              data={chartData()}
              options={chartOptions()}
            />
          </div>
            <p class="total">Total: { 
              apiResource().json.result.reduce(function(sum, i) { return sum + i.count; }, 0) }
            </p>
    {/*<LegendTable options={tableOptions()} />*/}
        </div>
      </Match>
    </Switch>
  )
}
