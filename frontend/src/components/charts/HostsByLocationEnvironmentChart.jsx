import { onMount } from 'solid-js'
import { Bar } from 'solid-chartjs'
import { createResource, For } from 'solid-js';
import { useParams, A } from "@solidjs/router";
import { getCoreRowModel } from '@tanstack/solid-table';
import { useContextMessage } from "../common/MessageProvider";
import JsonMessage from "../common/JsonMessage";
import { LegendTable } from '../common/Tables';
import { registerChartPlugins, barOptions, CHART_COLORS } from './chartConfig';

export default function HostsByLocationEnvironmentChart() {
  // const [message, setMessage] = useContextMessage();

  const apiUrl = '/api/hosts/groupby/location/environment';

  const apiFetch = async () => {
    const f = await fetch(apiUrl);
    const j = await f.json();
    return {fetch: f, json: j};
  }

  // const apiFetch = async () => await fetch(apiUrl);
  // const apiJson = async (f) => await f.json();

  const [apiResource] = createResource(apiUrl, apiFetch);
  // const [jsonResource] = createResource(apiResource, apiJson);

  // var columns = [
  //   { accessorKey: 'environment', header: 'Environment', cell: v => v.getValue(), class: 'text-align-left', footer: 'Environment' },
  //   // { accessorKey: 'total', header: 'Total', cell: v => v.getValue(), class: 'text-align-right', footer: 'Total' },
  //   { accessorKey: 'facts_key', header: 'facts_key', cell: v => v.getValue(), class: 'text-align-right', footer: 'facts_key' },
  //   { accessorKey: 'count', header: 'count', cell: v => v.getValue(), class: 'text-align-right', footer: 'count' },
  // ];

  // const tableOptions = () => {
  //   return {
  //     data: apiResource().json.result,
  //     columns: columns,
  //     getCoreRowModel: getCoreRowModel(),
  //   };
  // }

  onMount(() => registerChartPlugins())

  const fallback = () => {
    return (<div><p>Chart is not available</p></div>)
  }

  const chartOptions = () => barOptions('Hosts in Environment by Location', 'Hosts');


  var data = {};
  const chartData = () => {
    var environments = {};
    var locations = {};
    
    for (var i in apiResource().json.result) {
      var r = apiResource().json.result[i];
      if (! (r.environment in environments)) {
        environments[r.environment] = 0;
      }
      if (! (r.location in locations)) {
        locations[r.location] = 0;
      }
      if (! (r.environment in data)) {
        data[r.environment] = {};
      }
      data[r.environment][r.location] = r.count;
    }
    
    var environments_a = Object.keys(environments).sort();
    var locations_a = Object.keys(locations).sort();

    var colors = CHART_COLORS;
    var datasets = []; 
    for (var i = 0; i < environments_a.length; i++) {
      var env = environments_a[i];
      var data_a = [];
      for (var j = 0; j < locations_a.length; j++) {
        var loc = locations_a[j];
        if (! (loc in data[env])) {
          data_a.push(0);
        } else {
          data_a.push(data[env][loc]);
        }
      }
      datasets.push(
        { 
          label: environments_a[i],
          backgroundColor: colors[i % 13],
          data: data_a,
          stack: '0'
        }
      );
    }
    // console.debug(datasets);
    return {
        labels: locations_a,
        datasets: datasets,
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
          <div style="height: 400px; width: 900px;">
            <Bar
              fallback={fallback()}
              data={chartData()}
              options={chartOptions()}
            />
          </div>
        </div>
      </Match>
    </Switch>
  )
}
