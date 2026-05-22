import { useContextMessage } from "./MessageProvider";


export default function JsonMessage(props) { 
  const [message, setMessage] = useContextMessage();
  // if (props.message['error']) {
  //   setMessage("Error: " + props.message['error']);
  // } else {
  //   setMessage(JSON.stringify(props.message))
  // }
    setMessage(props.message);
  return;
}