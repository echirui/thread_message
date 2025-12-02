import { useState, useEffect } from 'react';
import useWebSocket from 'react-use-websocket';
import { client } from '../api/client';
import type { Message } from './useChat';

export const useThread = (threadId: number | null, senderId: string) => {
  const [messages, setMessages] = useState<Message[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const wsUrl = import.meta.env.VITE_API_URL 
    ? import.meta.env.VITE_API_URL.replace(/^http/, 'ws') + '/ws'
    : 'ws://localhost:3000/ws';

  const { lastMessage } = useWebSocket(wsUrl, {
    shouldReconnect: () => true,
    reconnectAttempts: 10,
    reconnectInterval: 3000,
  });

  useEffect(() => {
    if (threadId === null) {
      // eslint-disable-next-line react-hooks/set-state-in-effect
      setMessages([]);
      return;
    }

    setLoading(true);
    setError(null);
    client.get<Message[]>(`/messages/${threadId}/replies`)
      .then(res => {
        // eslint-disable-next-line react-hooks/set-state-in-effect
        setMessages(res.data);
      })
      .catch(err => {
        console.error("Failed to fetch thread messages", err);
        setError("Failed to load thread messages.");
      })
      .finally(() => {
        setLoading(false);
      });
  }, [threadId]);

  useEffect(() => {
    if (lastMessage !== null) {
      try {
        const newMsg = JSON.parse(lastMessage.data) as Message;
        if (newMsg.parent_id === threadId) {
          // eslint-disable-next-line react-hooks/set-state-in-effect
          setMessages((prev) => [...prev, newMsg]);
        }
      } catch (e) {
        console.error("Failed to parse WS message in useThread", e);
      }
    }
  }, [lastMessage, threadId]);

  const sendMessage = async (content: string) => {
    if (threadId === null) return;
    try {
        await client.post('/messages', { content, sender_id: senderId, parent_id: threadId });
    } catch (e) {
        console.error("Failed to send thread message", e);
        setError("Failed to send message.");
    }
  };

  return { messages, loading, error, sendMessage };
};