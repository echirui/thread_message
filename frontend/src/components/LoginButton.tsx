import { client } from '../api/client';

export const LoginButton = () => {
  const handleLogin = () => {
    const baseUrl = client.defaults.baseURL || 'http://localhost:3000';
    window.location.href = `${baseUrl}/auth/login`;
  };

  return (
    <button
      onClick={handleLogin}
      className="bg-gray-900 hover:bg-gray-700 text-white font-bold py-2 px-6 rounded shadow transition duration-200 flex items-center gap-2"
    >
      <span>Sign in with GitHub</span>
    </button>
  );
};
