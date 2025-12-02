import clsx from 'clsx';
import type { Message } from '../hooks/useChat'; // type を追加
import { useEffect, useRef } from 'react';

interface MessageListProps {
  messages: Message[];
  currentUserId: string;
  onMessageClick?: (messageId: number) => void;
}

export const MessageList = ({ messages, currentUserId, onMessageClick }: MessageListProps) => {
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  return (
    <div className="flex-1 overflow-y-auto p-4 space-y-4 bg-gray-50">
      {messages.map((msg) => {
        const isMyMessage = msg.sender_id === currentUserId;
        const isParentMessage = msg.parent_id === null;

        return (
          <div
            key={msg.id}
            className={clsx(
              "flex w-full",
              isMyMessage ? "justify-end" : "justify-start"
            )}
          >
            <div
              className={clsx(
                "max-w-[70%] rounded-2xl px-4 py-2 text-sm shadow-sm",
                isMyMessage
                  ? "bg-blue-500 text-white rounded-tr-none"
                  : "bg-white text-gray-800 border border-gray-200 rounded-tl-none",
                isParentMessage && onMessageClick && "cursor-pointer hover:bg-gray-100"
              )}
              onClick={isParentMessage && onMessageClick ? () => onMessageClick(msg.id) : undefined}
            >
              {!isMyMessage && (
                <div className="text-xs text-gray-400 mb-1 truncate">
                  {msg.sender_id}
                </div>
              )}
              <div className="break-words whitespace-pre-wrap">{msg.content}</div>
              {isParentMessage && onMessageClick && (
                <div className="text-xs text-blue-200 mt-1 cursor-pointer">
                  Reply to thread...
                </div>
              )}
            </div>
          </div>
        );
      })}
      <div ref={bottomRef} />
    </div>
  );
};