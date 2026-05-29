import { onMount } from 'solid-js'
import { Line } from 'solid-chartjs'
import { createResource } from 'solid-js';
import { useContextMessage } from "../common/MessageProvider";
import JsonMessage from "../common/JsonMessage";
import { registerChartPlugins, lineOptions } from './chartConfig';


export default function HardwareHistory() {
  // const [message, setMessage] = useContextMessage();

  // const apiFetch = async () => await fetch('/api/hosts/groupby/hardware/history');
  // const apiJson = async (f) => await f.json();

  const apiFetch = async () => {
    const f = await fetch('/api/hosts/groupby/hardware/history');
    const j = await f.json();
    return {fetch: f, json: j};
  }

  const [apiResource] = createResource(apiFetch);
  // const [jsonResource] = createResource(apiResource, apiJson);

  onMount(() => registerChartPlugins())

  const fallback = () => {
    return (<div><p>Chart is not available</p></div>)
  }

  const chartOptions = () => lineOptions('Hosts Hardware Evolution', 'Week', 'Hosts');

  const chartData = () => {
    var sets = {};
    for (var i in apiResource().json.result) {
      var r = apiResource().json.result[i];
      if (! (r.key in sets)) {
        sets[r.key] = [];
      }
      sets[r.key].push({ x: r.gathered_at_week, y: r.value });
    }
    var datasets = [];
    for (var i in sets) {
      datasets.push({ label: i, data: sets[i] });
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
        <div class="report" style="width:1100px;">
        <Line fallback={fallback()}
          data={chartData()}
          options={chartOptions()}
        />
        </div>
      </Match>
    </Switch>
  )
}
