import { MessageList } from './MessageList';
import { ChatInput } from './ChatInput';
import { useThread } from '../hooks/useThread';

interface ThreadViewProps {
  threadId: number | null;
  onClose: () => void;
  currentUserId: string; 
}

export const ThreadView = ({ threadId, onClose, currentUserId }: ThreadViewProps) => {
  const { messages, loading, error, sendMessage } = useThread(threadId, currentUserId); // sendMessage を追加

  const handleSend = (content: string) => {
    sendMessage(content);
  };

  if (threadId === null) {
    return null; // スレッドが選択されていない場合は表示しない
  }

  return (
    <div className="absolute inset-y-0 right-0 w-96 bg-white border-l border-gray-200 flex flex-col z-20 shadow-lg">
      <header className="flex justify-between items-center p-4 border-b border-gray-200">
        <h2 className="text-lg font-semibold">Thread Replies</h2>
        <button onClick={onClose} className="text-gray-500 hover:text-gray-700">
          ✕
        </button>
      </header>
      <div className="flex-1 flex flex-col overflow-hidden">
        {loading && <div className="p-4 text-center text-gray-500">Loading thread...</div>}
        {error && <div className="p-4 text-center text-red-500">{error}</div>}
        {!loading && !error && messages.length === 0 && (
          <div className="p-4 text-center text-gray-500">No replies yet.</div>
        )}
        <MessageList messages={messages} currentUserId={currentUserId} />
      </div>
      <ChatInput onSend={handleSend} disabled={loading || error !== null} />
    </div>
  );
};