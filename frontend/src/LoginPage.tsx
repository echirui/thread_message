import { LoginButton } from './components/LoginButton';

export const LoginPage = () => {
    return (
        <div className="h-screen w-full flex flex-col items-center justify-center bg-gray-50">
            <div className="bg-white p-10 rounded-xl shadow-lg text-center max-w-md w-full">
                <h1 className="text-3xl font-bold mb-6 text-gray-800">Thread Message</h1>
                <p className="mb-8 text-gray-600">
                    Welcome! Please sign in with your GitHub account to join the conversation.
                </p>
                <div className="flex justify-center">
                    <LoginButton />
                </div>
            </div>
        </div>
    );
};
