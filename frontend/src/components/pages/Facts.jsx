import { MessageProvider } from '../common/MessageProvider';
import Navigation from './Navigation';
import Message from './Message';

import { useParams, Navigate } from "@solidjs/router";
import { useContextMessage } from "../common/MessageProvider";
import FactsLinks from "../common/FactsLinks";
import FactsTable from "../common/FactsTable";

import { Switch } from 'solid-js';

export default function Facts() { 
  const [message, setMessage] = useContextMessage();

  const showTable = (!(typeof useParams().fkey === 'undefined'));

  return (
    <MessageProvider>
      <Navigation />
      <div class="contents">
      <Message />
      <FactsLinks />
      <Switch>
        <Match when={showTable}>
        <div class="flex">
          <FactsTable />
        </div>
        </Match>
        <Match when={!showTable}>
          <Navigate href="/facts/os_version" />
        </Match>
      </Switch>
      </div>
    </MessageProvider>
  )
}
