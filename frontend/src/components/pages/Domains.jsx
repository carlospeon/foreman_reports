import { MessageProvider } from '../common/MessageProvider';
import Navigation from './Navigation';
import Message from './Message';
import DomainsChart from '../charts/DomainsChart';
import DomainsHistory from '../charts/DomainsHistory';

export default function Domains() {

  return (
    <MessageProvider>
      <Navigation />
      <div class="contents">
        <Message />
        <div class="flex">
          <DomainsChart/>
          <DomainsHistory/>
        </div>
      </div>
    </MessageProvider>
  )
}
