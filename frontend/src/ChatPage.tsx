import { useState } from 'react';
import { MessageList } from './components/MessageList';
import { ChatInput } from './components/ChatInput';
import { ThreadView } from './components/ThreadView';
import { useChat } from './hooks/useChat';
import { useAuth } from './hooks/useAuth';

const ChatPage = () => {
  const { user } = useAuth();
  const userId = user?.id.toString() || 'unknown';
  const { messages, sendMessage, isConnected } = useChat(userId);
  const [selectedThreadId, setSelectedThreadId] = useState<number | null>(null);

  const handleOpenThread = (messageId: number) => {
    setSelectedThreadId(messageId);
  };

  const handleCloseThread = () => {
    setSelectedThreadId(null);
  };

  return (
    <div className="flex h-full w-full">
      {/* メインチャットエリア */}
      <div className="flex flex-col flex-1 min-w-0">
        <header className="bg-white border-b border-gray-200 p-4 flex justify-between items-center shadow-sm z-10">
          <h1 className="font-bold text-gray-800 text-lg"># general</h1>
          <div className="flex items-center gap-2 text-xs text-gray-500">
             <span className={`w-2 h-2 rounded-full ${isConnected ? 'bg-green-500' : 'bg-red-500'}`}></span>
             {isConnected ? 'Connected' : 'Disconnected'}
          </div>
        </header>

        {/* Messages Area */}
        <MessageList messages={messages} currentUserId={userId} onMessageClick={handleOpenThread} />

        {/* Input Area */}
        <ChatInput onSend={sendMessage} disabled={!isConnected} />
      </div>

      {/* Thread View */}
      {selectedThreadId !== null && (
        <ThreadView
          key={selectedThreadId}
          threadId={selectedThreadId}
          onClose={handleCloseThread}
          currentUserId={userId}
        />
      )}
    </div>
  );
};

export default ChatPage;