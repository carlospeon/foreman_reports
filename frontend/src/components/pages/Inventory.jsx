import { MessageProvider } from '../common/MessageProvider';
import Navigation from './Navigation';
import Message from './Message';

import { createResource, createEffect } from 'solid-js';
import { A } from "@solidjs/router";
import { useContextMessage } from "../common/MessageProvider";
import { ReportTable } from "../common/Tables";
import JsonMessage from "../common/JsonMessage";
import { capitalize, formatNumber } from "../common/Util";

import {
  flexRender,
  getCoreRowModel,
  createSolidTable,
} from '@tanstack/solid-table';


export default function Inventory() {
  
  const apiFetch = async () => {
    const f = await fetch('/api/report/os');
    const j = await f.json();
    return {fetch: f, json: j};
  }

  const [apiResource] = createResource(apiFetch);

  var columns = [
    { accessorKey: 'hostname', header: v => capitalize(v.column.id), cell: v => v.getValue(), 
      class: 'text-align-left', footer: v => capitalize(v.column.id), },
    { accessorKey: 'bu', header: v => capitalize(v.column.id), cell: v => v.getValue(), 
      class: 'text-align-left', footer: v => capitalize(v.column.id), },
    { accessorKey: 'comment', header: () => 'Description', cell: v => v.getValue(), 
      class: 'text-align-left overflow', footer: () => 'Description', },
    { accessorKey: 'facts_datetime', header: 'Facts date', cell: v => v.getValue(), 
      class: 'text-align-left', footer: 'Facts date', },
    { accessorKey: 'location', header: v => capitalize(v.column.id), cell: v => v.getValue(), 
      class: 'text-align-left', footer: v => capitalize(v.column.id), },
    { accessorKey: 'environment', header: v => capitalize(v.column.id), cell: v => v.getValue(), 
      class: 'text-align-left', footer: v => capitalize(v.column.id), },
    { accessorKey: 'subnet', header: v => capitalize(v.column.id), cell: v => v.getValue(), 
      class: 'text-align-left overflow', footer: v => capitalize(v.column.id), },
    { accessorKey: 'product_name', header: 'Hardware', cell: v => v.getValue(), 
      class: 'text-align-left overflow', footer: 'Hardware', },
    { accessorKey: 'os_version', header: 'OS', cell: v => v.getValue(), 
      class: 'text-align-right', footer: 'OS', },
    { accessorKey: 'kernel_release', header: 'Kernel', cell: v => v.getValue(), 
      class: 'text-align-left', footer: 'Kernel', },
    { accessorKey: 'sockets', header: 'Sockets', cell: v => v.getValue(), 
      class: 'text-align-right', footer: 'Sockets', },
    { accessorKey: 'cpu', header: 'CPU', cell: v => v.getValue(), 
      class: 'text-align-right', footer: 'CPU', },
    { accessorKey: 'memorysize', header: 'Memory (GB)', cell: v => v.getValue(), 
      class: 'text-align-right', footer: 'Memory (GB)', },
    { accessorKey: 'dfsize', header: 'Disk (GB)', cell: v => v.getValue(), 
      class: 'text-align-right', footer: 'Disk (GB)', },
  ];

  const tableOptions = () => {
    return {
      data: apiResource().json.result,
      columns: columns,
      getCoreRowModel: getCoreRowModel(),
    };
  }

  return (
    <MessageProvider>
      <Navigation />
      <div class="contents">
        <Message />
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
            <div class="links float-right">
            <a target="_self" href="/api/report/os?accept=csv">Donwload CSV</a>
            </div>
            <div>
            <ReportTable options={tableOptions()}/>
            </div>
          </Match>
        </Switch>
      </div>
    </MessageProvider>
  )
}

