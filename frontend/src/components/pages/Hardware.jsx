import { MessageProvider } from '../common/MessageProvider';
import Navigation from './Navigation';
import Message from './Message';
import HardwareChart from '../charts/HardwareChart';
import HardwareHistory from '../charts/HardwareHistory';

export default function Hardware() {

  return (
    <MessageProvider>
      <Navigation />
      <div class="contents">
        <Message />
        <div class="report">
          <HardwareChart/>
          {/* <HardwareHistory/> */}
        </div>
      </div>
    </MessageProvider>
  )
}
