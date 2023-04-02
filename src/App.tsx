import ChatContainer from "./componests/chatContainer";
import { Input } from "antd";

const { Search } = Input;

function App() {
	return (
		<>
			<ChatContainer />
			<div>正在与机器人聊天中</div>
			<div>
				<Search className="chat-search" placeholder="请输入内容" />
			</div>
		</>
	);
}

export default App;
