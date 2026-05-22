import { onMount } from 'solid-js'
import { Chart, Title, Tooltip, Legend, Colors } from 'chart.js'
import { Bar, Doughnut } from 'solid-chartjs'
import { createResource, For } from 'solid-js';
import { getCoreRowModel } from '@tanstack/solid-table';
import { useContextMessage } from "../common/MessageProvider";
import JsonMessage from "../common/JsonMessage";
import { LegendTable } from '../common/Tables';

export default function UpdatesChart(props) {
  const [message, setMessage] = useContextMessage();

  var apiUrl = '/api/updated';
  if (typeof(props.ts) !== 'undefined') {
    apiUrl = apiUrl + "/" + props.ts;
  }

  const apiFetch = async () => {
    const f = await fetch(apiUrl);
    const j = await f.json();
    return {fetch: f, json: j};
  }
  // const apiFetch = async () => await fetch(apiUrl);
  // const apiJson = async (f) => await f.json();

  const [apiResource] = createResource(apiFetch);
  // const [jsonResource] = createResource(apiResource, apiJson);


  var columns = [
    { accessorKey: 'total', header: 'Total', cell: v => v.getValue(), class: 'padding-left text-align-right', footer: 'Total' },
    { accessorKey: 'supported', header: 'Supported', cell: v => v.getValue(), class: 'padding-left text-align-right', footer: 'Suported' },
    { accessorKey: 'supported_p', header: '%', cell: v => "(" + v.getValue() + "%)", class: '', footer: '%' },
    { accessorKey: 'updated', header: 'Updated', cell: v => v.getValue(), class: 'padding-left text-align-right', footer: 'Updated' },
    { accessorKey: 'updated_p', header: '%', cell: v => "(" + v.getValue() + "%)", class: '', footer: '%' },
    // { accessorKey: 'total', header: 'Total', cell: v => v.getValue(), class: 'text-align-right',
    //   footer: ({table}) => table.getFilteredRowModel().rows.reduce((total, row) => total + row.getValue('count'), 0),
    // },
  ];

  const tableOptions = () => {
    return {
      data: apiResource().json.result,
      columns: columns,
      getCoreRowModel: getCoreRowModel(),
    };
  }

  onMount(() => {
    Chart.register(Title, Tooltip, Legend, Colors);
    Chart.defaults.font.size = 13;
  })

  const fallback = () => {
    return (<div><p>Chart is not available</p></div>)
  }

  const chartOptions = {
    responsive: true,
    maintainAspectRatio: false,
    scales: {
      y: { title: { display: true, text: 'Hosts' }, stacked: true},
    },
    plugins: {
      title: {
        display: true,
        text: 'Total'
      },
      legend: {
        display: true,
        position: 'bottom',
      },
      colors: {
        enabled: true,
      }
    },
  }

  const chartData = () => {
    return {
      labels: ['Total'],
        datasets: [
            {
              label: 'Supported',
              data: apiResource().json.result.map(function(i) { return i.supported; }),
              backgroundColor: '#36a2eb',
              stack: '0',
            },
            {
              label: 'Non Supported',
              data: apiResource().json.result.map(function(i) { return i.total - i.supported; }),
              backgroundColor: '#dddddd',
              stack: '0',
            },
            {
              label: 'Updated',
              data: apiResource().json.result.map(function(i) { return i.updated; }),
              backgroundColor: '#ff6384',
              stack: '1',
            },
            {
              label: 'Non Updated',
              data: apiResource().json.result.map(function(i) { return i.total - i.updated; }),
              backgroundColor: '#dddddd',
              stack: '1',
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
            <Bar 
              fallback={fallback()}
              data={chartData()}
              options={chartOptions}
            />
          </div>
          <div>
            <LegendTable options={tableOptions()} />
          </div>
        </div>
      </Match>
    </Switch>
  )
}
