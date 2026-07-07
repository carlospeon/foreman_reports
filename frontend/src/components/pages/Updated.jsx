import { useParams, A } from "@solidjs/router";
import { MessageProvider } from '../common/MessageProvider';
import Navigation from './Navigation';
import Message from './Message';
import UpdatedChart from '../charts/UpdatedChart';
import UpdatedByEnvironmentChart from '../charts/UpdatedByEnvironmentChart';
import UpdatedByLocationChart from '../charts/UpdatedByLocationChart';
import NonUpdatedByAdminEnvironmentChart from '../charts/NonUpdatedByAdminEnvironmentChart';

export default function Updates() {
  const params = useParams();
  const ts = params.ts;

  return (
    
    <MessageProvider>
      <Navigation />
      <div class="contents">
        <Message />
        <div class="links">
          <A activeClass="active" end="true" href="/updated">Current</A>
          <A activeClass="active" href="/updated/compliant">Compliant</A>
        </div>
        <div class="flex">
          <UpdatedChart ts={ts} />
          <UpdatedByLocationChart ts={ts} />
          <UpdatedByEnvironmentChart ts={ts} />
          <NonUpdatedByAdminEnvironmentChart ts={ts} />
        </div>
      </div>
    </MessageProvider>
  )
}
