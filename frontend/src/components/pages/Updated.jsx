import { useParams, A } from "@solidjs/router";
import { MessageProvider } from '../common/MessageProvider';
import Navigation from './Navigation';
import Message from './Message';
import OSChart from '../charts/OSChart';
import UpdatedChart from '../charts/UpdatedChart';
import OSHistory from '../charts/OSHistory';
import UpdatedByEnvironmentChart from '../charts/UpdatedByEnvironmentChart';
import UpdatedByLocationChart from '../charts/UpdatedByLocationChart';
import NonUpdatedByAdminEnvironmentChart from '../charts/NonUpdatedByAdminEnvironmentChart';
import OSEOLTable from '../charts/OSEOLTable';

export default function Updates() {
  const params = useParams();
  const ts = params.ts;

  return (
    
    <MessageProvider>
      <Navigation />
      <div class="contents">
        <div class="links">
          <A activeClass="active" end="true" href="/updated">Current</A>
          <A activeClass="active" href="/updated/compliant">Compliant</A>
        </div>
        <Message />
        <div class="flex">
          <UpdatedChart ts={ts} />
          <UpdatedByLocationChart ts={ts} />
          <UpdatedByEnvironmentChart ts={ts} />
          <NonUpdatedByAdminEnvironmentChart ts={ts} />
        </div>
        <div class="flex">
          <OSChart/>
          <OSHistory />
          <OSEOLTable />
        </div>
      </div>
    </MessageProvider>
  )
}
