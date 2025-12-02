import { useState } from 'react';
import { MessageList } from './components/MessageList';
import { ChatInput } from './components/ChatInput';
import { useChat } from './hooks/useChat';

// Simple ID generator for demo
const getUserId = () => {
  let id = localStorage.getItem('thread_msg_user_id');
  if (!id) {
    id = 'user_' + Math.random().toString(36).substring(2, 9);
    localStorage.setItem('thread_msg_user_id', id);
  }
  return id;
};

const ChatPage = () => {
  const [userId] = useState(getUserId());
  const { messages, sendMessage, isConnected } = useChat(userId);

  return (
    <div className="flex flex-col h-full w-full">
      {/* Header */}
      <header className="bg-white border-b border-gray-200 p-4 flex justify-between items-center shadow-sm z-10">
        <h1 className="font-bold text-gray-800 text-lg"># general</h1>
        <div className="flex items-center gap-2 text-xs text-gray-500">
           <span className={`w-2 h-2 rounded-full ${isConnected ? 'bg-green-500' : 'bg-red-500'}`}></span>
           {isConnected ? 'Connected' : 'Disconnected'}
        </div>
      </header>

      {/* Messages Area */}
      <MessageList messages={messages} currentUserId={userId} />

      {/* Input Area */}
      <ChatInput onSend={sendMessage} disabled={!isConnected} />
    </div>
  );
};

export default ChatPage;