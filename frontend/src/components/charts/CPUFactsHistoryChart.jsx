import { onMount } from 'solid-js'
import { Chart, Title, Tooltip, Legend, Colors } from 'chart.js'
import { Line } from 'solid-chartjs'
import { createResource } from 'solid-js';
import { useParams } from "@solidjs/router";
import { useContextMessage } from "../common/MessageProvider";
import JsonMessage from "../common/JsonMessage";


export default function CPUFactsHistoryChart() {

   const getApiUrl = () =>  {
    var url = '/api/report/facts';
    if (! (typeof useParams().fkey === 'undefined')) {
      url += '/' + encodeURIComponent(useParams().fkey);
    }
    if (! (typeof useParams().fvalue === 'undefined')) {
      url += '/' + encodeURIComponent(useParams().fvalue);
    }
    return url + '/cpu/history';
  }

  const fkey = () => useParams().fkey;
  const [apiUrl, setApiUrl] = createResource(fkey, getApiUrl);
  
  const apiFetch = async () => {
    const f = await fetch(apiUrl());
    const j = await f.json();
    return {fetch: f, json: j};
  }

  // const [apiResource] = createResource(apiFetch);
  const [apiResource, {mutate, refetch}] = createResource(apiUrl, apiFetch);


  onMount(() => {
    Chart.register(Title, Tooltip, Legend, Colors);
    Chart.defaults.font.size = 13;
  })

  const fallback = () => {
    return (<div><p>Chart is not available</p></div>)
  }

  Chart.defaults.font.size = 13;
  const chartOptions = {
    responsive: true,
    maintainAspectRatio: false,
    scales: {
      x: { title: { display: true, text: 'Week' }, },
      y: { title: { display: true, text: 'Online CPU' }, 
           suggestedMin: 0, },
    },
    plugins: {
      title: {
        display: true,
        text: 'CPUs by Fact Evolution'
      },
      legend: {
        position: 'bottom',
      },
    },
  }

  const chartData = () => {
    var sets = {};
    for (var i in apiResource().json.result) {
      var r = apiResource().json.result[i];
      if (r.key == fkey() ) {
        if (! (r.key in sets)) {
          sets[r.key] = [];
        }
        sets[r.key].push({ x: r.gathered_at_week, y: r.value });
      }
    }
    var datasets = [];
    for (var i in sets) {
      datasets.push({ label: i, data: sets[i], 
        pointBackgroundColor: 'rgba(0, 0, 0, 0)', 
        pointBorderColor: 'rgba(0, 0, 0, 0)',
        pointRadius: 5
      });
    }
    return {
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
        <div class="report" style="height: 500px; width: 700px;">
        <Line fallback={fallback()} 
          data={chartData()} 
          options={chartOptions}
        />
        </div>
      </Match>
    </Switch>
  )
}
