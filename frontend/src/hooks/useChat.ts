import { useState, useEffect } from 'react';
import useWebSocket from 'react-use-websocket';
import { client } from '../api/client';

export interface Message {
  id: number;
  content: string;
  sender_id: string;
  created_at: string;
}

export const useChat = (senderId: string) => {
  const [messages, setMessages] = useState<Message[]>([]);
  const [isConnected, setIsConnected] = useState(false);
  
  // Load initial history
  useEffect(() => {
    client.get<Message[]>('/messages')
      .then(res => {
          setMessages(res.data);
      })
      .catch(err => console.error("Failed to fetch messages", err));
  }, []);

  // WebSocket
  const wsUrl = import.meta.env.VITE_API_URL 
    ? import.meta.env.VITE_API_URL.replace(/^http/, 'ws') + '/ws'
    : 'ws://localhost:3000/ws';

  const { lastMessage, readyState } = useWebSocket(wsUrl, {
    shouldReconnect: () => true,
    reconnectAttempts: 10,
    reconnectInterval: 3000,
  });

  useEffect(() => {
      setIsConnected(readyState === 1); // WebSocket.OPEN
  }, [readyState]);

  useEffect(() => {
    if (lastMessage !== null) {
      try {
        const newMsg = JSON.parse(lastMessage.data) as Message;
        setMessages((prev) => [...prev, newMsg]);
      } catch (e) {
        console.error("Failed to parse WS message", e);
      }
    }
  }, [lastMessage]);

  const sendMessage = async (content: string) => {
    try {
        await client.post('/messages', { content, sender_id: senderId });
        // Message will be received via WebSocket
    } catch (e) {
        console.error("Failed to send message", e);
    }
  };

  return { messages, sendMessage, isConnected };
};
