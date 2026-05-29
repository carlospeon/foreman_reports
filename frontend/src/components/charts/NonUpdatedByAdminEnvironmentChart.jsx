import { onMount } from 'solid-js'
import { Bar, Bubble } from 'solid-chartjs'
import { createResource, For } from 'solid-js';
import { getCoreRowModel } from '@tanstack/solid-table';
import { useContextMessage } from "../common/MessageProvider";
import JsonMessage from "../common/JsonMessage";
import { LegendTable } from '../common/Tables';
import { registerChartPlugins, getThemeColors } from './chartConfig';

export default function NonUpdatedByAdminEnvironmentChart(props) {
  // const [message, setMessage] = useContextMessage();

  var apiUrl = '/api/nonupdated/groupby/adminenvironment';
  if (typeof(props.ts) !== 'undefined') {
    apiUrl = apiUrl + "/" + props.ts;
  }
  // const apiFetch = async () => await fetch(apiUrl);
  // const apiJson = async (f) => await f.json();
  const apiFetch = async () => {
    const f = await fetch(apiUrl);
    const j = await f.json();
    return {fetch: f, json: j};
  }

  const [apiResource] = createResource(apiFetch);
  // const [jsonResource] = createResource(apiResource, apiJson);


  onMount(() => registerChartPlugins())

  const fallback = () => {
    return (<div><p>Chart is not available</p></div>)
  }

  const chartOptions = () => {
    const colors = getThemeColors();
    return {
      responsive: true,
      maintainAspectRatio: false,
      barPercentage: 1.0,
      categoryPercentage: 1.0,
      scales: {
        x: { ticks: { color: colors.textSecondary }, grid: { color: colors.grid } },
        y: { title: { display: false }, ticks: { display: false }, stacked: true, grid: { color: colors.grid } },
      },
      plugins: {
        title: { display: true, text: 'Non Updated heat map', color: colors.text },
        legend: { display: false },
        tooltip: {
          callbacks: {
            label: function(context) {
              var d = data[context.dataset.label][context.label] || 0;
              return context.dataset.label + ': ' + d;
            }
          }
        }
      },
    };
  };

  function dataBackgroundColors(valuesArray) {
    var colors = [];
    var rgb = "0, 175, 221";
    var rgb = "54, 162, 235";
  
    for (var i in valuesArray) {
      var value = valuesArray[i];
      var opacity = Math.log10(value + 1) / 3.2;
      if (opacity > 1) {opacity = 1};
  
      colors.push("rgba(" + rgb + ", " + opacity + ")");
    }
  
    return colors;
  }
  
  var data = {};
  const chartData = () => {
    var admins = {};
    var environments = {};
    
    for (var i in apiResource().json.result) {
      var r = apiResource().json.result[i];
      if (! (r.admin in admins)) {
        admins[r.admin] = 0;
      }
      if (! (r.environment in environments)) {
        environments[r.environment] = 0;
      }
      if (! (r.environment in data)) {
        data[r.environment] = {};
      }
      data[r.environment][r.admin] = r.non_updated;
    }
    
    var admins_a = Object.keys(admins).sort();
    var environments_a = Object.keys(environments).sort();

    var datasets = []; 
    for (var i = 0; i < environments_a.length; i++) {
      var env = environments_a[i];
      var data_a = [];
      for (var j = 0; j < admins_a.length; j++) {
        var adm = admins_a[j];
        if (! (adm in data[env])) {
          data_a.push(0);
        } else {
          data_a.push(data[env][adm]);
        }
      }
      datasets.push(
        { 
          label: environments_a[i],
          backgroundColor: dataBackgroundColors(data_a),
          data: Array(admins_a.length).fill(1),
          stack: '0'
        }
      );
    }
    // console.debug(datasets);
    return {
        labels: admins_a,
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
          <div style="height: 300px; width: 500px;">
            <Bar
              fallback={fallback()}
              data={chartData()}
              options={chartOptions()}
            />
          </div>
          {/* <div>
            <LegendTable options={tableOptions()} />
          </div> */}
        </div>
      </Match>
    </Switch>
  )
}
