import { onMount } from 'solid-js'
import { Chart, Title, Tooltip, Legend, Colors } from 'chart.js'
import { Bar } from 'solid-chartjs'
import { createResource, For } from 'solid-js';
import { useParams, A } from "@solidjs/router";
import { getCoreRowModel } from '@tanstack/solid-table';
import { useContextMessage } from "../common/MessageProvider";
import JsonMessage from "../common/JsonMessage";
import { LegendTable } from '../common/Tables';
import { createEffect } from 'solid-js';

export default function FactsByEnvironmentChart() {
  // const [message, setMessage] = useContextMessage();

  const getApiUrl = () =>  {
    var url = '/api/report/facts';
    if (! (typeof useParams().fkey === 'undefined')) {
      url += '/' + encodeURIComponent(useParams().fkey);
    }
    if (! (typeof useParams().fvalue === 'undefined')) {
      url += '/' + encodeURIComponent(useParams().fvalue);
    }
    return url + '/groupby/environment';
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
    // { accessorKey: 'total', header: 'Total', cell: v => v.getValue(), class: 'text-align-right', footer: 'Total' },
    { accessorKey: 'facts_key', header: 'facts_key', cell: v => v.getValue(), class: 'text-align-right', footer: 'facts_key' },
    { accessorKey: 'count', header: 'count', cell: v => v.getValue(), class: 'text-align-right', footer: 'count' },
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
      y: { title: { display: true, text: 'Hosts' }, stacked: true },
    },
    plugins: {
      title: {
        display: true,
        text: 'Facts'
      },
      // colors: {
      //   enabled: true,
      //   forceOverride: true
      // },
      legend: {
        position: 'bottom',
      },
    },
  }


  var data = {};
  const chartData = () => {
    var fkeys = {};
    var environments = {};
    
    for (var i in apiResource().json.result) {
      var r = apiResource().json.result[i];
      if (! (r.facts_key in fkeys)) {
        fkeys[r.facts_key] = 0;
      }
      if (! (r.environment in environments)) {
        environments[r.environment] = 0;
      }
      if (! (r.facts_key in data)) {
        data[r.facts_key] = {};
      }
      data[r.facts_key][r.environment] = r.count;
    }
    
    var fkeys_a = Object.keys(fkeys).sort();
    var environments_a = Object.keys(environments).sort();

    var colors = ['#36a2eb', '#ff6384', '#4bc0c0', '#ff9f40', '#96f', '#ffcd56', '#c9cbcf',
                  '#e1b496', '#82d2f5', '#9be1af', '#e1d2af', '#4b646e', '#dc3282' ];
    var datasets = []; 
    for (var i = 0; i < fkeys_a.length; i++) {
      var fkey = fkeys_a[i];
      var data_a = [];
      for (var j = 0; j < environments_a.length; j++) {
        var env = environments_a[j];
        if (! (env in data[fkey])) {
          data_a.push(0);
        } else {
          data_a.push(data[fkey][env]);
        }
      }
      datasets.push(
        { 
          label: fkeys_a[i],
          backgroundColor: colors[i % 13],
          data: data_a,
          stack: '0'
        }
      );
    }
    // console.debug(datasets);
    return {
        labels: environments_a,
        datasets: datasets,
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
              options={chartOptions}
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
