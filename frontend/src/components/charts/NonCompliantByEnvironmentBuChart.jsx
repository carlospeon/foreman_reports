import { onMount } from 'solid-js'
import { Bar } from 'solid-chartjs'
import { createResource, For } from 'solid-js';
import { useParams, A } from "@solidjs/router";
import { getCoreRowModel } from '@tanstack/solid-table';
import { useContextMessage } from "../common/MessageProvider";
import JsonMessage from "../common/JsonMessage";
import { LegendTable } from '../common/Tables';
import { createEffect } from 'solid-js';
import { registerChartPlugins, barOptions, CHART_COLORS } from './chartConfig';

export default function NonCompliantByEnvironmentBuChart(props) {
  // const [message, setMessage] = useContextMessage();

  var apiUrl = '/api/hosts/erratas/groupby/environment/bu';
  if (typeof(props.ts) !== 'undefined') {
    apiUrl = apiUrl + "/noncompliant/" + props.ts;
  }
  const apiFetch = async () => {
    const f = await fetch(apiUrl);
    const j = await f.json();
    return {fetch: f, json: j};
  }

  //const [apiResource, {mutate, refetch}] = createResource(apiUrl, apiFetch);
  const [apiResource] = createResource(apiFetch);


  var columns = [
    { accessorKey: 'environment', header: 'Environment', cell: v => v.getValue(), class: 'text-align-left', footer: 'Environment' },
    // { accessorKey: 'total', header: 'Total', cell: v => v.getValue(), class: 'text-align-right', footer: 'Total' },
    { accessorKey: 'bu', header: 'BU', cell: v => v.getValue(), class: 'text-align-left', footer: 'BU' },
    { accessorKey: 'count', header: 'count', cell: v => v.getValue(), class: 'text-align-right', footer: 'count' },
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

  const chartOptions = () => barOptions('Non Compliant', 'Hosts');


  var data = {};
  const chartData = () => {
    var bus = {};
    var environments = {};
    
    for (var i in apiResource().json.result) {
      var r = apiResource().json.result[i];
      if (! (r.bu in bus)) {
        bus[r.bu] = 0;
      }
      if (! (r.environment in environments)) {
        environments[r.environment] = 0;
      }
      if (! (r.bu in data)) {
        data[r.bu] = {};
      }
      data[r.bu][r.environment] = r.count;
    }
    
    var bus_a = Object.keys(bus).sort();
    var environments_a = Object.keys(environments).sort();

    var colors = CHART_COLORS;
    var datasets = []; 
    for (var i = 0; i < bus_a.length; i++) {
      var bu = bus_a[i];
      var data_a = [];
      for (var j = 0; j < environments_a.length; j++) {
        var env = environments_a[j];
        if (! (env in data[bu])) {
          data_a.push(0);
        } else {
          data_a.push(data[bu][env]);
        }
      }
      datasets.push(
        { 
          label: bus_a[i],
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
