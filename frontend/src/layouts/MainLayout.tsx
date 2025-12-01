import { Outlet } from 'react-router-dom';

export const MainLayout = () => {
  return (
    <div className="flex h-screen w-screen overflow-hidden bg-gray-100">
      {/* Sidebar (Channel List / Navigation) */}
      <aside className="w-64 flex-shrink-0 border-r border-gray-200 bg-white hidden md:flex flex-col">
        <div className="p-4 font-bold text-lg border-b border-gray-200 text-gray-800">
          Thread Message
        </div>
        <div className="p-4 flex-1 overflow-y-auto">
          <div className="text-sm text-gray-500">Channels (Coming soon)</div>
        </div>
      </aside>

      {/* Main Content Area */}
      <main className="flex-1 flex flex-col min-w-0 bg-white relative">
         <Outlet />
      </main>
    </div>
  );
};
